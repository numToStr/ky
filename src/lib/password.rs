use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::OsRng;

use super::KyError;

pub struct Password {
    password: String,
}

impl Password {
    pub fn init() -> Result<Self, KyError> {
        let password = dialoguer::Password::new()
            .with_prompt("New master password")
            .with_confirmation("Retype to verify", "Passwords didn't match")
            .interact()?;

        Ok(Self { password })
    }

    pub fn hash(&self) -> Result<String, KyError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();

        let hash = argon
            .hash_password_simple(self.password.as_bytes(), salt.as_ref())
            .map_err(|_| KyError::Hashing)?
            .to_string();

        Ok(hash)
    }
}
