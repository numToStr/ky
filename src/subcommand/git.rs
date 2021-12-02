use std::fs::remove_dir_all;

use super::Command;
use crate::{
    check_db,
    cli::Config,
    lib::{Git, KyError, KyResult, Prompt},
};
use clap::{Parser, Subcommand};

macro_rules! check_git_details {
    ($repo: expr, $branch: expr) => {{
        match ($repo, $branch) {
            (Some(repo), Some(branch)) => Ok((repo, branch)),
            (None, _) => Err(KyError::GitRepo),
            (_, None) => Err(KyError::GitBranch),
        }
    }};
}

#[derive(Debug, Parser)]
pub struct GitCmd {
    #[clap(subcommand)]
    cmd: GitSubcmd,
}

impl Command for GitCmd {
    fn exec(&self, config: Config) -> KyResult<()> {
        match &self.cmd {
            GitSubcmd::Init(c) => c.exec(config),
            GitSubcmd::Backup(c) => c.exec(config),
            GitSubcmd::Restore(c) => c.exec(config),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum GitSubcmd {
    /// Initialize a git repo in the vault directory
    Init(GitInit),

    /// Backup the vault to the git repository
    #[clap(visible_alias = "push")]
    Backup(GitBackup),

    /// Restore vault from a git repository
    #[clap(visible_alias = "clone")]
    Restore(GitRestore),
}

#[derive(Debug, Parser)]
pub struct GitInit {
    /// Push after initialization
    #[clap(short, long)]
    push: bool,
}

impl Command for GitInit {
    fn exec(&self, config: Config) -> KyResult<()> {
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

#[derive(Debug, Parser)]
pub struct GitBackup {
    /// Force push
    #[clap(short, long)]
    force: bool,

    /// Push without committing
    #[clap(short, long)]
    no_commit: bool,
}

impl Command for GitBackup {
    fn exec(&self, config: Config) -> KyResult<()> {
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

#[derive(Debug, Parser)]
pub struct GitRestore {
    /// Ignore already initialized vault, if any
    #[clap(short = 'I', long)]
    ignore: bool,
}

impl Command for GitRestore {
    fn exec(&self, config: Config) -> KyResult<()> {
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
