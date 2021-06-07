use super::Command;
use crate::{
    check_git_details,
    cli::Config,
    lib::{Database, Git, KyError, Password, Prompt, MASTER},
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Init {
    /// Use `git` to track vault changes
    #[clap(short = 'G', long)]
    git: bool,
}

impl Command for Init {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        if db_path.exists() {
            return Err(KyError::Init);
        }

        // If --git is set then check the git details before vault is initiliazed
        // Otherwise vault will initiliazed but git will return if details is not set
        let (repo, branch) = if self.git {
            check_git_details!(config.git_repo, config.git_branch)?
        } else {
            ("".to_string(), "".to_string())
        };

        let db = Database::init(&db_path)?;

        let password = Password::init(&Prompt::theme())?;

        let hashed = password.hash()?;

        db.set(MASTER, &hashed)?;

        // If --git is set, then do init + add + commit
        if self.git {
            Git::new(&repo, &branch, &db_path).init()?.add()?.commit()?;
        }

        Ok(())
    }
}
