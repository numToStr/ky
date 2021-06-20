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
    pub fn new(env: &Env, name: &KyTable) -> Result<Self, KyError> {
        let db: KyDbType = env
            .create_database(Some(&name.to_string()))
            .map_err(|_| KyError::Connection)?;

        Ok(Self { db })
    }

    pub fn set(&self, wtxn: &mut RwTxn, key: &str, val: &str) -> Result<(), KyError> {
        let res = self
            .db
            .put(wtxn, key, val)
            .map_err(|_| KyError::Set(key.to_string()))?;

        Ok(res)
    }

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

    pub fn delete(&self, wtxn: &mut RwTxn, key: &str) -> Result<bool, KyError> {
        let is_deleted = self
            .db
            .delete(wtxn, key)
            .map_err(|_| KyError::Delete(key.to_string()))?;

        Ok(is_deleted)
    }

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
    pub fn connect(path: &Path) -> Result<Self, KyError> {
        let env = EnvOpenOptions::new()
            .max_dbs(5)
            .open(path)
            .map_err(|_| KyError::Connection)?;

        Ok(Self { env })
    }

    pub fn get_table(&self, name: KyTable) -> Result<KyDb, KyError> {
        KyDb::new(&self.env, &name)
    }

    pub fn write_txn(&self) -> Result<RwTxn, KyError> {
        let wtxn = self.env.write_txn()?;

        Ok(wtxn)
    }

    pub fn read_txn(&self) -> Result<RoTxn, KyError> {
        let rtxn = self.env.read_txn()?;

        Ok(rtxn)
    }

    pub fn close(self) {
        self.env.prepare_for_closing().wait();
    }
}

pub struct Database {
    env: Env,
    conn: KyDbType,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, KyError> {
        let env = EnvOpenOptions::new()
            .max_dbs(5)
            .open(path)
            .map_err(|_| KyError::Connection)?;

        let table = KyTable::Password.to_string();

        // we will open the default unamed database
        let conn: KyDbType = env
            .create_database(Some(&table))
            .map_err(|_| KyError::Connection)?;

        Ok(Self { env, conn })
    }

    pub fn write_txn(&self) -> Result<RwTxn, KyError> {
        let wtxn = self.env.write_txn()?;

        Ok(wtxn)
    }

    pub fn read_txn(&self) -> Result<RoTxn, KyError> {
        let rtxn = self.env.read_txn()?;

        Ok(rtxn)
    }

    pub fn set(&self, wtxn: &mut RwTxn, key: &str, val: &str) -> Result<(), KyError> {
        let res = self
            .conn
            .put(wtxn, key, val)
            .map_err(|_| KyError::Set(key.to_string()))?;

        Ok(res)
    }

    pub fn get(&self, rtxn: &RoTxn, key: &str) -> Result<String, KyError> {
        let bytes = self
            .conn
            .get(&rtxn, key)
            .map_err(|_| KyError::Get(key.to_string()))?;

        match bytes {
            Some(x) => Ok(x.to_string()),
            _ => Err(KyError::NotFound(key.to_string())),
        }
    }

    pub fn delete(&self, wtxn: &mut RwTxn, key: &str) -> Result<bool, KyError> {
        let is_deleted = self
            .conn
            .delete(wtxn, key)
            .map_err(|_| KyError::Delete(key.to_string()))?;

        Ok(is_deleted)
    }

    pub fn ls(&self, rtxn: &RoTxn) -> Result<Vec<(String, String)>, KyError> {
        let mut keys: Vec<(String, String)> = Vec::new();

        for kv in self.conn.iter(rtxn)? {
            let (k, v) = kv?;

            if k != MASTER {
                keys.push((k.to_string(), v.to_string()));
            }
        }

        Ok(keys)
    }

    pub fn close(self) {
        self.env.prepare_for_closing().wait();
    }
}
