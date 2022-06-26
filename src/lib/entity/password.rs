use std::{convert::TryFrom, fmt::Display};

use crate::{
    cli::PasswordParams,
    lib::{Cipher, Decrypted, Encrypted, KyError, KyResult},
};

use rand::{thread_rng, Rng};

pub const DELIM: char = ':';

const CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~`-_+=><.,:;'\"[]{}?/\\|";

#[derive(Debug)]
pub struct Password {
    pub password: String,
    pub username: String,
    pub website: String,
    pub expires: String,
    pub note: String,
}

impl Password {
    pub fn generate(opts: &PasswordParams) -> String {
        let chars: Vec<u8> = {
            let charset = match &opts.charset {
                Some(x) => x.as_bytes(),
                None => CHARSET,
            };

            match &opts.exclude {
                Some(x) => {
                    let excluded = x.as_bytes();

                    charset
                        .iter()
                        .copied()
                        .filter(|c| !excluded.contains(c))
                        .collect()
                }
                _ => charset.to_vec(),
            }
        };

        let len = chars.len();
        let mut rng = thread_rng();

        (0..opts.length)
            .map(|_| {
                let idx = rng.gen_range(0..len);
                chars[idx] as char
            })
            .collect()
    }

    #[inline]
    fn dehex(s: Option<&str>) -> KyResult<String> {
        let dehexed = hex::decode(s.unwrap()).map_err(|_| KyError::Decrypt)?;
        Ok(String::from_utf8_lossy(&dehexed).to_string())
    }

    #[deprecated = "Use self.to_string()"]
    pub fn encrypt(self, cipher: &Cipher) -> KyResult<Encrypted> {
        cipher.encrypt(&self.to_string().into())
    }

    #[deprecated = "Use Password::try_from(decrypted)"]
    pub fn decrypt(cipher: &Cipher, encrypted: &Encrypted) -> KyResult<Self> {
        let decrypted = cipher.decrypt(encrypted)?;

        Self::try_from(decrypted)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let password = hex::encode(&self.password);
        let username = hex::encode(&self.username);
        let website = hex::encode(&self.website);
        let expires = hex::encode(&self.expires);
        let note = hex::encode(&self.note);

        write!(
            f,
            "{password}{DELIM}{username}{DELIM}{website}{DELIM}{expires}{DELIM}{note}",
        )
    }
}

impl TryFrom<Decrypted> for Password {
    type Error = KyError;

    fn try_from(decrypted: Decrypted) -> Result<Self, Self::Error> {
        let mut keys = decrypted.as_ref().splitn(5, DELIM);

        let password = Self::dehex(keys.next())?;
        let username = Self::dehex(keys.next())?;
        let website = Self::dehex(keys.next())?;
        let expires = Self::dehex(keys.next())?;
        let note = Self::dehex(keys.next())?;

        Ok(Self {
            password,
            username,
            website,
            expires,
            note,
        })
    }
}
