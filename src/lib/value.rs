use super::{Cipher, KyError};
use std::fmt::{self, Display, Formatter};

pub const DELIM: char = ':';

#[macro_use]
macro_rules! dehexed {
    ($k: expr) => {{
        match $k.next() {
            Some(x) => {
                if x == "" {
                    x.to_string()
                } else {
                    let dehexed = hex::decode(x).map_err(|_| crate::lib::KyError::Decrypt)?;
                    String::from_utf8_lossy(&dehexed).to_string()
                }
            }
            _ => "".to_string(),
        }
    }};
}

#[macro_use]
macro_rules! hexed {
    ($k: expr) => {{
        match $k.as_str() {
            "" => $k.to_string(),
            x => hex::encode(x),
        }
    }};
}

#[derive(Debug)]
pub struct Value {
    pub password: String,
    pub username: String,
    pub website: String,
    pub expires: String,
    pub notes: String,
}

impl Value {
    pub fn encrypt(&self, cipher: &Cipher) -> Result<String, KyError> {
        let password = hexed!(cipher.encrypt(&self.password)?);
        let username = hexed!(self.username);
        let website = hexed!(self.website);
        let expires = hexed!(self.expires);
        let notes = hexed!(self.notes);

        let val = Value {
            password,
            username,
            website,
            expires,
            notes,
        }
        .to_string();

        cipher.encrypt(&val)
    }

    pub fn decrypt(cipher: &Cipher, encrypted: &str) -> Result<Self, KyError> {
        let decrypted = cipher.decrypt(&encrypted)?;

        let mut keys = decrypted.splitn(5, DELIM);

        let password = dehexed!(keys);
        let username = dehexed!(keys);
        let website = dehexed!(keys);
        let expires = dehexed!(keys);
        let notes = dehexed!(keys);

        Ok(Self {
            password,
            username,
            website,
            expires,
            notes,
        })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{d}{}{d}{}{d}{}{d}{}",
            self.password,
            self.username,
            self.website,
            self.expires,
            self.notes,
            d = DELIM,
        )
    }
}
