use super::Command;
use crate::{
    check_db,
    cli::Config,
    lib::{Git, KyError},
};
use clap::Clap;

#[macro_use]
macro_rules! check_git_details {
    ($repo: expr, $branch: expr) => {{
        match ($repo, $branch) {
            (Some(repo), Some(branch)) => Ok((repo, branch)),
            (None, _) => Err(KyError::GitRepo),
            (_, None) => Err(KyError::GitBranch),
        }
    }};
}

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

        if db_path.join(".git").exists() {
            return Err(KyError::GitInit);
        }

        let (repo, branch) = check_git_details!(config.git_repo, config.git_branch)?;

        Git::new(&repo, &branch, &db_path)
            .init()?
            .add()?
            .commit()?
            .push()?;

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
