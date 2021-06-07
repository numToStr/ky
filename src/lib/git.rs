use super::KyError;
use std::{
    path::Path,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Git<'a> {
    repo: &'a str,
    branch: &'a str,
    target: String,
}

impl<'a> Git<'a> {
    pub fn new(repo: &'a str, branch: &'a str, target: &'a Path) -> Self {
        Self {
            repo,
            branch,
            target: target.display().to_string(),
        }
    }

    #[inline]
    fn git(&self) -> Command {
        Command::new("git")
    }

    pub fn init(self) -> Result<Self, KyError> {
        let init_status = self.git().args(&["init", &self.target]).spawn()?.wait()?;

        if !init_status.success() {
            return Err(KyError::Git);
        }

        Ok(self)
    }

    pub fn add(self) -> Result<Self, KyError> {
        let status = self
            .git()
            .args(&["-C", &self.target, "add", "."])
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(KyError::Git);
        }

        Ok(self)
    }

    pub fn commit(self) -> Result<Self, KyError> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|x| KyError::Any(x.to_string()))?
            .as_secs()
            .to_string();

        let status = self
            .git()
            .args(&[
                "-C",
                &self.target,
                "commit",
                "-m",
                &format!("backup @ {}", time),
            ])
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(KyError::Git);
        }

        Ok(self)
    }

    pub fn push(self) -> Result<Self, KyError> {
        let status = self
            .git()
            .args(&["-C", &self.target, "push", &self.repo, &self.branch])
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(KyError::Git);
        }

        Ok(self)
    }
}
