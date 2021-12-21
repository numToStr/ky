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
    pub notes: String,
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
                        .to_vec()
                        .into_iter()
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

    pub fn encrypt(self, cipher: &Cipher) -> KyResult<Encrypted> {
        let password = hex::encode(self.password);
        let username = hex::encode(self.username);
        let website = hex::encode(self.website);
        let expires = hex::encode(self.expires);
        let notes = hex::encode(self.notes);

        let val = format!(
            "{}{d}{}{d}{}{d}{}{d}{}",
            password,
            username,
            website,
            expires,
            notes,
            d = DELIM,
        );

        cipher.encrypt(&Decrypted::from(val))
    }

    pub fn decrypt(cipher: &Cipher, encrypted: &Encrypted) -> KyResult<Self> {
        let decrypted: String = cipher.decrypt(encrypted)?.into();

        let mut keys = decrypted.splitn(5, DELIM);

        let password = Self::dehex(keys.next())?;
        let username = Self::dehex(keys.next())?;
        let website = Self::dehex(keys.next())?;
        let expires = Self::dehex(keys.next())?;
        let notes = Self::dehex(keys.next())?;

        Ok(Self {
            password,
            username,
            website,
            expires,
            notes,
        })
    }
}
