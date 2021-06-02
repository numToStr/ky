use std::path::PathBuf;

use super::KyError;
use rocksdb::{Options, DB};

pub struct Database {
    conn: DB,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self, KyError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_keep_log_file_num(1);
        opts.set_skip_checking_sst_file_sizes_on_db_open(true);

        let conn = DB::open(&opts, path).map_err(|_| KyError::Connection)?;

        Ok(Self { conn })
    }

    pub fn exist(&self, key: &'static str) -> Result<bool, KyError> {
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

    // pub fn delete(&self, key: &'static str) -> Result<(), KyError> {
    //     let res = self.conn.delete(key).map_err(|_| KyError::Delete(key))?;
    //
    //     Ok(res)
    // }
}
