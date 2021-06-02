use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
pub struct Config {
    #[clap(long, env = "KY_DB")]
    db_path: Option<PathBuf>,
}

impl Config {
    pub fn db_path(self) -> PathBuf {
        self.db_path
            .unwrap_or_else(|| PathBuf::new().join("lok.db"))
    }
}
