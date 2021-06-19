use super::KyError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryKey(String);

impl FromStr for EntryKey {
    type Err = KyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<String> for EntryKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for EntryKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
