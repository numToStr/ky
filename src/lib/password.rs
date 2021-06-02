use std::fmt::{self, Display, Formatter};

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::{rngs::OsRng, thread_rng, Rng};

const CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~`-_+=><:;'[]{}?/";

use super::KyError;

pub struct Password {
    raw: String,
}

impl Password {
    pub fn init() -> Result<Self, KyError> {
        let raw = dialoguer::Password::new()
            .with_prompt("New master password")
            .with_confirmation("Retype to verify", "Passwords didn't match")
            .interact()?;

        Ok(Self { raw })
    }

    pub fn ask_master() -> Result<Self, KyError> {
        let raw = dialoguer::Password::new()
            .with_prompt("Enter master password")
            .interact()?;

        Ok(Self { raw })
    }

    pub fn hash(&self) -> Result<String, KyError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon = Argon2::default();

        let hash = argon
            .hash_password_simple(self.raw.as_bytes(), salt.as_ref())
            .map_err(|_| KyError::Hashing)?
            .to_string();

        Ok(hash)
    }

    pub fn verify(&self, hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).unwrap();

        let argon = Argon2::default();

        argon
            .verify_password(self.raw.as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn generate(len: u64) -> Self {
        let mut rng = thread_rng();

        let raw: String = (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
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
