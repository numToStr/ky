use super::{KyError, MASTER};
use rocksdb::{IteratorMode, Options, DB};
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

pub struct Database {
    conn: DB,
}

impl Database {
    fn options() -> Options {
        let mut opts = Options::default();
        opts.set_keep_log_file_num(1);
        opts.set_skip_checking_sst_file_sizes_on_db_open(true);

        opts
    }

    pub fn init(path: &Path) -> Result<Self, KyError> {
        let mut opts = Self::options();
        opts.create_if_missing(true);

        let conn = DB::open(&opts, path).map_err(|_| KyError::Connection)?;

        Ok(Self { conn })
    }

    pub fn open(path: &Path) -> Result<Self, KyError> {
        let opts = Self::options();

        let conn = DB::open(&opts, path).map_err(|_| KyError::Connection)?;

        Ok(Self { conn })
    }

    pub fn exist(&self, key: &str) -> Result<bool, KyError> {
        Ok(self
            .conn
            .get_pinned(key)
            .map_err(|x| KyError::Any(x.to_string()))?
            .is_some())
    }

    pub fn get(&self, key: &str) -> Result<String, KyError> {
        let bytes = self
            .conn
            .get(key)
            .map_err(|_| KyError::Get(key.to_string()))?;

        match bytes {
            Some(x) => {
                let s = String::from_utf8(x)
                    .map_err(|_| KyError::Any("Unable to parse value".to_string()))?;

                Ok(s)
            }
            _ => Err(KyError::NotFound(key.to_string())),
        }
    }

    pub fn set(&self, key: &str, val: &str) -> Result<(), KyError> {
        let res = self
            .conn
            .put(key, val)
            .map_err(|_| KyError::Set(key.to_string()))?;

        Ok(res)
    }

    pub fn ls(&self) -> Vec<String> {
        let mut keys = Vec::new();

        for i in self.conn.iterator(IteratorMode::End) {
            let key = String::from_utf8(i.0.to_vec()).expect("Invalid key");

            if key != *MASTER {
                keys.push(key);
            }
        }

        keys
    }

    pub fn delete(&self, key: &str) -> Result<(), KyError> {
        let res = self
            .conn
            .delete(key)
            .map_err(|_| KyError::Delete(key.to_string()))?;

        Ok(res)
    }
}
