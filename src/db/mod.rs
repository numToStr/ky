use rocksdb::{Options, DB};

pub struct Database {
    conn: DB,
}

impl Database {
    const PATH: &'static str = "lok.db";

    pub fn new() -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_keep_log_file_num(1);
        opts.set_skip_checking_sst_file_sizes_on_db_open(true);

        let conn = DB::open(&opts, Self::PATH).expect("Unable to establish database connection");

        Self { conn }
    }

    pub fn get(&self, key: &str) -> String {
        String::from_utf8(self.conn.get(key).unwrap().unwrap()).unwrap()
    }

    pub fn set(&self, key: &str, val: &str) {
        self.conn.put(key, val).unwrap();
    }

    pub fn delete(&self, key: &str) {
        self.conn.delete(key).unwrap();
    }
}
