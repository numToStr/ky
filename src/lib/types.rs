use super::KyError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[macro_use]
macro_rules! asref_str {
    ($struct: ty) => {
        impl AsRef<str> for $struct {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

#[macro_use]
macro_rules! from_string {
    ($struct: ty) => {
        impl From<String> for $struct {
            fn from(s: String) -> Self {
                Self(s)
            }
        }
    };
}

/// EntryKey represent the entry key in raw form
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryKey(String);

asref_str!(EntryKey);

impl FromStr for EntryKey {
    type Err = KyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

/// Encrypted represent a value which is in encypted form
pub struct Encrypted(String);

asref_str!(Encrypted);

from_string!(Encrypted);

impl<'a> From<&'a str> for Encrypted {
    /// This is mostly for converting MASTER as an encrypted type
    fn from(s: &'a str) -> Self {
        Self(s.to_string())
    }
}

impl From<Encrypted> for String {
    fn from(s: Encrypted) -> Self {
        s.0
    }
}

/// Decrypted represent a value that in raw or decrypted form
pub struct Decrypted(String);

asref_str!(Decrypted);

from_string!(Decrypted);

impl From<Decrypted> for String {
    fn from(s: Decrypted) -> Self {
        s.0
    }
}

impl From<Decrypted> for EntryKey {
    fn from(s: Decrypted) -> Self {
        Self(s.0)
    }
}

impl From<&EntryKey> for Decrypted {
    fn from(s: &EntryKey) -> Self {
        Self(s.0.to_owned())
    }
}
