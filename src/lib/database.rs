use super::{KyError, MASTER};
use heed::{types::Str, Database as Mdbx, Env, EnvOpenOptions, RoTxn, RwTxn};
use std::{
    fmt::{self, Display, Formatter},
    path::Path,
};

/// Just a check to ensure that the database exist before connecting
#[macro_export]
macro_rules! check_db {
    ($path: expr) => {
        if !$path.exists() {
            return Err(crate::lib::KyError::NoInit);
        }
    };
}

type KyDbType = Mdbx<Str, Str>;

/// KyTable is a collection of all the table names
pub enum KyTable {
    Master,
    Password,
}

impl Display for KyTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Master => f.write_str("master"),
            Self::Password => f.write_str("password"),
        }
    }
}

/// KyDb holds a individual database and its methods
pub struct KyDb {
    db: KyDbType,
}

impl KyDb {
    /// Returns a new connection to a database from provided env and name
    pub fn new(env: &Env, name: &KyTable) -> Result<Self, KyError> {
        let db: KyDbType = env
            .create_database(Some(&name.to_string()))
            .map_err(|_| KyError::Connection)?;

        Ok(Self { db })
    }

    /// Insert a key-val pair into the databse
    pub fn set(&self, wtxn: &mut RwTxn, key: &str, val: &str) -> Result<(), KyError> {
        let res = self
            .db
            .put(wtxn, key, val)
            .map_err(|_| KyError::Set(key.to_string()))?;

        Ok(res)
    }

    /// Retrieve a key-val pair from the databse
    pub fn get(&self, rtxn: &RoTxn, key: &str) -> Result<String, KyError> {
        let bytes = self
            .db
            .get(&rtxn, key)
            .map_err(|_| KyError::Get(key.to_string()))?;

        match bytes {
            Some(x) => Ok(x.to_string()),
            _ => Err(KyError::NotFound(key.to_string())),
        }
    }

    /// Delete a key-val pair from the databse
    pub fn delete(&self, wtxn: &mut RwTxn, key: &str) -> Result<bool, KyError> {
        let is_deleted = self
            .db
            .delete(wtxn, key)
            .map_err(|_| KyError::Delete(key.to_string()))?;

        Ok(is_deleted)
    }

    /// Retrieve all the key-val pair in the databse
    pub fn ls(&self, rtxn: &RoTxn) -> Result<Vec<(String, String)>, KyError> {
        let mut keys: Vec<(String, String)> = Vec::new();

        for kv in self.db.iter(rtxn)? {
            let (k, v) = kv?;

            if k != MASTER {
                keys.push((k.to_string(), v.to_string()));
            }
        }

        Ok(keys)
    }
}

/// KyEnv hold the lmdb environment and database connection
pub struct KyEnv {
    env: Env,
}

impl KyEnv {
    /// Connects to new lmdb environment
    pub fn connect(path: &Path) -> Result<Self, KyError> {
        let env = EnvOpenOptions::new()
            .max_dbs(5)
            .open(path)
            .map_err(|_| KyError::Connection)?;

        Ok(Self { env })
    }

    /// Returns a database connection
    pub fn get_table(&self, name: KyTable) -> Result<KyDb, KyError> {
        KyDb::new(&self.env, &name)
    }

    /// Returns a write-read transaction
    pub fn write_txn(&self) -> Result<RwTxn, KyError> {
        let wtxn = self.env.write_txn()?;

        Ok(wtxn)
    }

    /// Returns a read-only transaction
    pub fn read_txn(&self) -> Result<RoTxn, KyError> {
        let rtxn = self.env.read_txn()?;

        Ok(rtxn)
    }

    /// Closes the open connection
    pub fn close(self) {
        self.env.prepare_for_closing().wait();
    }
}
