use super::LokError;
use rocksdb::{Options, DB};

pub struct Database {
    conn: DB,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, LokError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_keep_log_file_num(1);
        opts.set_skip_checking_sst_file_sizes_on_db_open(true);

        let conn = DB::open(&opts, path).map_err(|_| LokError::Connection)?;

        Ok(Self { conn })
    }

    pub fn get(&self, key: &'static str) -> Result<String, LokError> {
        let bytes = self.conn.get(key).map_err(|_| LokError::Get(key))?;

        match bytes {
            Some(x) => {
                let s = String::from_utf8(x)
                    .map_err(|_| LokError::Any("Unable to parse value".to_string()))?;

                Ok(s)
            }
            _ => Err(LokError::NotFound(key)),
        }
    }

    pub fn set(&self, key: &'static str, val: &str) -> Result<(), LokError> {
        let res = self.conn.put(key, val).map_err(|_| LokError::Set(key))?;

        Ok(res)
    }

    pub fn delete(&self, key: &'static str) -> Result<(), LokError> {
        let res = self.conn.delete(key).map_err(|_| LokError::Delete(key))?;

        Ok(res)
    }
}
