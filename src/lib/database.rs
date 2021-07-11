use super::{Encrypted, KyError, KyResult, MASTER};
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
    /// Common table to store common data across multiple tables ie. master password
    Common,
    /// Password table where all the passwords are stored
    Password,
}

impl Display for KyTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Common => f.write_str("common"),
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
    pub fn new(env: &Env, name: &KyTable) -> KyResult<Self> {
        let db: KyDbType = env
            .create_database(Some(&name.to_string()))
            .map_err(|_| KyError::Connection)?;

        Ok(Self { db })
    }

    /// Insert a key-val pair into the databse
    pub fn set(&self, wtxn: &mut RwTxn, key: &Encrypted, val: &Encrypted) -> KyResult<()> {
        let res = self
            .db
            .put(wtxn, key.as_ref(), val.as_ref())
            .map_err(|_| KyError::Set)?;

        Ok(res)
    }

    /// Retrieve a key-val pair from the databse
    pub fn get(&self, rtxn: &RoTxn, key: &Encrypted) -> KyResult<Encrypted> {
        let bytes = self.db.get(&rtxn, key.as_ref()).map_err(|_| KyError::Get)?;

        match bytes {
            Some(x) => Ok(Encrypted::from(x)),
            _ => Err(KyError::NotFound),
        }
    }

    /// Delete a key-val pair from the databse
    pub fn delete(&self, wtxn: &mut RwTxn, key: &Encrypted) -> KyResult<bool> {
        let is_deleted = self
            .db
            .delete(wtxn, key.as_ref())
            .map_err(|_| KyError::Delete)?;

        Ok(is_deleted)
    }

    /// Retrieve all the key-val pair in the databse
    pub fn ls(&self, rtxn: &RoTxn) -> KyResult<Vec<(Encrypted, Encrypted)>> {
        let mut keys: Vec<(Encrypted, Encrypted)> = Vec::new();

        for kv in self.db.iter(rtxn)? {
            let (k, v) = kv?;

            if k != MASTER {
                keys.push((Encrypted::from(k), Encrypted::from(v)));
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
    pub fn connect(path: &Path) -> KyResult<Self> {
        let env = EnvOpenOptions::new()
            .max_dbs(5)
            .open(path)
            .map_err(|_| KyError::Connection)?;

        Ok(Self { env })
    }

    /// Returns a database connection
    pub fn get_table(&self, name: KyTable) -> KyResult<KyDb> {
        KyDb::new(&self.env, &name)
    }

    /// Returns a write-read transaction
    pub fn write_txn(&self) -> KyResult<RwTxn> {
        let wtxn = self.env.write_txn()?;

        Ok(wtxn)
    }

    /// Returns a read-only transaction
    pub fn read_txn(&self) -> KyResult<RoTxn> {
        let rtxn = self.env.read_txn()?;

        Ok(rtxn)
    }

    /// Closes the open connection
    pub fn close(self) {
        self.env.prepare_for_closing().wait();
    }
}
