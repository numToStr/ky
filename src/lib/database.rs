use super::{KyError, MASTER};
use heed::{types::Str, Database as Mdbx, Env, EnvOpenOptions, RoTxn, RwTxn};
use std::path::Path;

/// Just a check to ensure that the database exist before connecting
#[macro_export]
macro_rules! check_db {
    ($path: expr) => {
        if !$path.exists() {
            return Err(crate::lib::KyError::NoInit);
        }
    };
}

type DatabaseType = Mdbx<Str, Str>;

pub struct Database {
    env: Env,
    conn: DatabaseType,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, KyError> {
        let env = EnvOpenOptions::new()
            .open(path)
            .map_err(|_| KyError::Connection)?;

        // we will open the default unamed database
        let conn: DatabaseType = env.create_database(None).map_err(|_| KyError::Connection)?;

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
