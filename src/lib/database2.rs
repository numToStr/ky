use super::{KyError, MASTER};
use mdbx::{
    Database as Mdbx, Environment, GenericEnvironment, NoWriteMap, Transaction, WriteFlags, RW,
};
use std::path::Path;

type Env = GenericEnvironment<NoWriteMap>;
type Txn<'db> = Transaction<'db, RW, NoWriteMap>;

pub struct Database2<'db> {
    txn: &'db Txn<'db>,
    conn: Mdbx<'db>,
}

impl<'db> Database2<'db> {
    pub fn env(path: &Path) -> Result<Env, KyError> {
        let env = Environment::new()
            .open(path)
            .map_err(|_| KyError::Connection)?;

        Ok(env)
    }

    pub fn open(txn: &'db Txn) -> Result<Self, KyError> {
        // we will open the default unamed database
        let conn = txn.open_db(None).map_err(|_| KyError::Connection)?;

        Ok(Self { txn, conn })
    }

    // pub fn write_txn(&self) -> Result<RwTxn, KyError> {
    //     let wtxn = self.env.write_txn()?;
    //
    //     Ok(wtxn)
    // }
    //
    // pub fn read_txn(&self) -> Result<RoTxn, KyError> {
    //     let rtxn = self.env.read_txn()?;
    //
    //     Ok(rtxn)
    // }

    pub fn set(&self, key: &str, val: &str) -> Result<(), KyError> {
        let res = self
            .txn
            .put(&self.conn, key, val, WriteFlags::NO_DUP_DATA)
            .map_err(|_| KyError::Set(key.to_string()))?;

        Ok(res)
    }

    pub fn get(&self, key: &str) -> Result<String, KyError> {
        let bytes = self
            .txn
            .get(&self.conn, key)
            .map_err(|_| KyError::Get(key.to_string()))?;

        match bytes {
            Some(x) => Ok(String::from_utf8(x.to_vec()).unwrap()),
            _ => Err(KyError::NotFound(key.to_string())),
        }
    }

    pub fn delete(&self, key: &str, val: &str) -> Result<(), KyError> {
        self.txn
            .del(&self.conn, key, Some(val.as_bytes()))
            .map_err(|_| KyError::Delete(key.to_string()))?;

        Ok(())
    }

    pub fn ls(&self) -> Result<Vec<String>, KyError> {
        let mut keys = Vec::new();

        for kv in self.txn.cursor(&self.conn)? {
            let (k, _) = kv?;

            if k != MASTER {
                keys.push(String::from_utf8(k.to_vec()).unwrap());
            }
        }

        Ok(keys)
    }
}
