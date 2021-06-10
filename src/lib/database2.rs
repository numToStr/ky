use super::KyError;
use heed::{types::Str, Database, Env, EnvOpenOptions, RoTxn, RwTxn};
use std::path::Path;

type DatabaseType = Database<Str, Str>;

pub struct Database2 {
    env: Env,
    conn: DatabaseType,
}

impl Database2 {
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
}

// {
//         create_dir_all(Path::new(&config.vault_home()).join("bytemuck.mdb"))?;
//         let env = EnvOpenOptions::new()
//             .open(Path::new(&config.vault_home()).join("bytemuck.mdb"))
//             .unwrap();
//
//         // we will open the default unamed database
//         let db: Database<Str, OwnedType<i32>> = env.create_database(None).unwrap();
//
//         // opening a write transaction
//         let mut wtxn = env.write_txn().unwrap();
//         db.put(&mut wtxn, "seven", &7).unwrap();
//         db.put(&mut wtxn, "zero", &0).unwrap();
//         db.put(&mut wtxn, "five", &5).unwrap();
//         db.put(&mut wtxn, "three", &3).unwrap();
//         wtxn.commit().unwrap();
//
//         // opening a read transaction
//         // to check if those values are now available
//         let rtxn = env.read_txn().unwrap();
//
//         let ret = db.get(&rtxn, "zero").unwrap();
//         println!("{:#?}", ret);
//         // assert_eq!(ret, Some(0));
//
//         let ret = db.get(&rtxn, "five").unwrap();
//         println!("{:#?}", ret);
//         // assert_eq!(ret, Some(5));
//         Ok(())
//     }
