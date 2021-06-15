use std::fs::remove_dir_all;

use super::Command;
use crate::{
    check_db,
    cli::Config,
    lib::{Git, KyError, Prompt},
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

    /// Restore vault from a git repository
    Restore(GitRestore),
}

impl Command for GitCmd {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        match self {
            Self::Init(c) => c.exec(config),
            Self::Push(c) => c.exec(config),
            Self::Restore(c) => c.exec(config),
        }
    }
}

#[derive(Debug, Clap)]
pub struct GitInit {
    /// Push after initialization
    #[clap(short, long)]
    push: bool,
}

impl Command for GitInit {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        if db_path.join(".git").exists() {
            return Err(KyError::GitInit);
        }

        let (repo, branch) = check_git_details!(config.git_repo, config.git_branch)?;

        let git = Git::new(&repo, &branch, &db_path).init()?.add()?.commit()?;

        if self.push {
            git.push(false)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clap)]
pub struct GitPush {
    /// Force push
    #[clap(short, long)]
    force: bool,

    /// Push without committing
    #[clap(short, long)]
    no_commit: bool,
}

impl Command for GitPush {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        if !db_path.join(".git").exists() {
            return Err(KyError::GitNoInit);
        }

        let (repo, branch) = check_git_details!(config.git_repo, config.git_branch)?;

        let git = Git::new(&repo, &branch, &db_path);

        if self.no_commit {
            git.push(self.force)?;
        } else {
            git.add()?.commit()?.push(self.force)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clap)]
pub struct GitRestore {
    /// Ignore already initialized vault, if any
    #[clap(short = 'I', long)]
    ignore: bool,
}

impl Command for GitRestore {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let (repo, branch) = check_git_details!(&config.git_repo, &config.git_branch)?;

        let theme = Prompt::theme();

        let db_path = config.db_path();
        let db_exist = db_path.exists();

        if !self.ignore && db_exist && !Prompt::vault_exist(&theme)? {
            return Ok(());
        }

        if db_exist {
            remove_dir_all(&db_path)?;
        }

        Git::new(&repo, &branch, &db_path).clone()?;

        Ok(())
    }
}
