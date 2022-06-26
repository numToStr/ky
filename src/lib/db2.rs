use std::path::Path;

use redb::{
    Database, ReadOnlyTable, ReadTransaction, ReadableTable, Table, TableDefinition,
    WriteTransaction,
};

use super::{Encrypted, KyError, KyResult, KyTable};

const MASTER_TABLE: TableDefinition<str, str> = TableDefinition::new("master");
const PWD_TABLE: TableDefinition<str, str> = TableDefinition::new("password");

pub struct KyDb2 {
    db: Database,
}

impl<'db> KyDb2 {
    const SIZE: i32 = 1024 * 1024;

    /// Creates a new database connection
    pub fn new(path: &Path) -> KyResult<Self> {
        let db = unsafe { Database::create(path, Self::SIZE as usize)? };

        Ok(Self { db })
    }

    /// Connects to an existing database
    pub fn connect(path: &Path) -> KyResult<Self> {
        if !path.exists() {
            return Err(KyError::NoInit);
        }

        Self::new(path)
    }

    /// Create a write transaction
    pub fn wtxn(&self) -> KyResult<WriteTransaction> {
        let wtxn = self.db.begin_write()?;
        Ok(wtxn)
    }

    /// Create a read transaction
    pub fn rtxn(&self) -> KyResult<ReadTransaction> {
        let rtxn = self.db.begin_read()?;
        Ok(rtxn)
    }

    /// Open a table with write transaction
    pub fn open_write<'txn>(
        &self,
        wtxn: &'txn WriteTransaction<'db>,
        tbl: KyTable,
    ) -> KyResult<KyWriteTable<'db, 'txn>> {
        let table = match tbl {
            KyTable::Master => wtxn.open_table(MASTER_TABLE),
            KyTable::Password => wtxn.open_table(PWD_TABLE),
        }?;

        Ok(KyWriteTable(table))
    }

    /// Open a table with read transaction
    pub fn open_read<'txn>(
        &self,
        rtxn: &'txn ReadTransaction<'db>,
        tbl: KyTable,
    ) -> KyResult<KyReadTable<'txn>> {
        let table = match tbl {
            KyTable::Master => rtxn.open_table(MASTER_TABLE),
            KyTable::Password => rtxn.open_table(PWD_TABLE),
        }?;

        Ok(KyReadTable(table))
    }
}

pub struct KyReadTable<'r>(ReadOnlyTable<'r, str, str>);

impl<'r> KyReadTable<'r> {
    pub fn get(&self, key: &Encrypted) -> KyResult<Encrypted> {
        match self.0.get(key.as_ref())? {
            Some(x) => Ok(x.into()),
            _ => Err(KyError::NotFound),
        }
    }

    // pub fn entries(&self) -> KyResult<Vec<(Encrypted, Encrypted)>> {
    //     unimplemented!()
    // }
}

pub struct KyWriteTable<'db, 'txn>(Table<'db, 'txn, str, str>);

impl<'db, 'txn> KyWriteTable<'db, 'txn> {
    pub fn get(&self, key: &Encrypted) -> KyResult<Encrypted> {
        match self.0.get(key.as_ref())? {
            Some(x) => Ok(x.into()),
            _ => Err(KyError::NotFound),
        }
    }

    pub fn set(&mut self, key: Encrypted, value: Encrypted) -> KyResult<()> {
        self.0.insert(key.as_ref(), value.as_ref())?;
        Ok(())
    }
}
