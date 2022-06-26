use derive_more::{AsRef, From, FromStr, Into};
use serde::{Deserialize, Serialize};

use super::entity::Password;

/// EntryKey represent the entry key in raw form
#[derive(Debug, Clone, From, FromStr, AsRef, Serialize, Deserialize)]
#[from(forward)]
pub struct EntryKey(String);

/// Encrypted represent a value which is in encypted form
#[derive(Debug, AsRef, From, FromStr)]
#[from(forward)]
pub struct Encrypted(String);

/// Decrypted represent a value that in raw or decrypted form
#[derive(Debug, From, Into, AsRef)]
pub struct Decrypted(String);

impl From<EntryKey> for Decrypted {
    fn from(s: EntryKey) -> Self {
        Self(s.0)
    }
}

impl From<Password> for Decrypted {
    fn from(p: Password) -> Self {
        Self(p.to_string())
    }
}
