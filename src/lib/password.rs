use super::{KyError, KyResult};
use crate::cli::PasswordParams;
use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use dialoguer::theme::Theme;
use rand::{rngs::OsRng, thread_rng, Rng};
use std::fmt::{self, Display, Formatter};

const ITR: u32 = 3;
const MEM: u32 = 1024 * 128; // 128MB

const CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~`-_+=><.,:;'\"[]{}?/\\|";

pub struct Password {
    raw: String,
}

impl Password {
    pub fn init(theme: &impl Theme) -> KyResult<Self> {
        let raw = dialoguer::Password::with_theme(theme)
            .with_prompt("New master password")
            .with_confirmation("Retype to verify", "Passwords didn't match")
            .interact()?;

        Ok(Self { raw })
    }

    pub fn ask_master(theme: &impl Theme) -> KyResult<Self> {
        let raw = dialoguer::Password::with_theme(theme)
            .with_prompt("Enter master password")
            .interact()?;

        Ok(Self { raw })
    }

    pub fn hash(&self) -> KyResult<String> {
        let pll = num_cpus::get() as u32;
        let salt = SaltString::generate(&mut OsRng);

        // TODO: first arg can be replace by a key file
        let argon = Argon2::new(None, ITR, MEM, pll, Version::default())
            .map_err(|e| KyError::Any(e.to_string()))?;

        let hash = argon
            .hash_password_simple(self.raw.as_bytes(), salt.as_ref())
            .map_err(|_| KyError::PwdHash)?
            .to_string();

        Ok(hash)
    }

    pub fn verify(&self, hash: &str) -> KyResult<bool> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| KyError::PwdVerify)?;

        let argon = Argon2::default();

        let is_verified = argon
            .verify_password(self.raw.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_verified)
    }

    pub fn generate(opts: &PasswordParams) -> Self {
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

        let raw: String = (0..opts.length)
            .map(|_| {
                let idx = rng.gen_range(0..len);
                charset[idx] as char
            })
            .collect();

        Self { raw }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}
