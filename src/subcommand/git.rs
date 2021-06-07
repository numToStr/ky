use clap::Clap;

use crate::{
    check_db, check_git_details,
    cli::Config,
    lib::{Git, KyError},
};

use super::Command;

#[derive(Debug, Clap)]
pub enum GitCmd {
    /// Initialize a git repo in the vault directory
    Init(GitInit),

    /// Push the vault to the git repository
    Push(GitPush),
}

impl Command for GitCmd {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        match self {
            Self::Init(c) => c.exec(config),
            Self::Push(c) => c.exec(config),
        }
    }
}

#[derive(Debug, Clap)]
pub struct GitInit;

impl Command for GitInit {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let (repo, branch) = check_git_details!(config.git_repo, config.git_branch)?;

        Git::new(&repo, &branch, &db_path).init()?.add()?.commit()?;

        Ok(())
    }
}

#[derive(Debug, Clap)]
pub struct GitPush;

impl Command for GitPush {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let (repo, branch) = check_git_details!(config.git_repo, config.git_branch)?;

        Git::new(&repo, &branch, &db_path).add()?.commit()?.push()?;

        Ok(())
    }
}
