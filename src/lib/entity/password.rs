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
        let charset: Vec<u8> = match &opts.exclude {
            Some(x) => {
                let exclude_bytes = x.as_bytes();

                CHARSET
                    .to_vec()
                    .into_iter()
                    .filter(|c| !exclude_bytes.contains(c))
                    .collect()
            }
            _ => CHARSET.to_vec(),
        };

        let mut rng = thread_rng();

        let len = charset.len();

        let pwd: String = (0..opts.length)
            .map(|_| {
                let idx = rng.gen_range(0..len);
                charset[idx] as char
            })
            .collect();

        pwd
    }

    #[inline]
    fn hex(s: String) -> String {
        if s.is_empty() {
            s
        } else {
            hex::encode(s)
        }
    }

    #[inline]
    fn dehex(s: Option<&str>) -> KyResult<String> {
        let r = {
            let x = s.unwrap_or_default();

            if x.is_empty() {
                x.to_owned()
            } else {
                let dehexed = hex::decode(x).map_err(|_| KyError::Decrypt)?;
                String::from_utf8_lossy(&dehexed).to_string()
            }
        };

        Ok(r)
    }

    pub fn encrypt(self, cipher: &Cipher) -> KyResult<Encrypted> {
        let password = Self::hex(cipher.encrypt(&Decrypted::from(self.password))?.into());
        let username = Self::hex(self.username);
        let website = Self::hex(self.website);
        let expires = Self::hex(self.expires);
        let notes = Self::hex(self.notes);

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
