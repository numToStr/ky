use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use dialoguer::theme::Theme;
use rand::rngs::OsRng;

use crate::lib::{Encrypted, KyError, KyResult};

const ITR: u32 = 3;
const MEM: u32 = 1024 * 128; // 128MB

pub struct Master {
    raw: String,
}

impl Master {
    pub fn new(theme: &impl Theme) -> KyResult<Self> {
        let raw = dialoguer::Password::with_theme(theme)
            .with_prompt("New master password")
            .with_confirmation("Retype to verify", "Passwords didn't match")
            .interact()?;

        Ok(Self { raw })
    }

    pub fn ask(theme: &impl Theme) -> KyResult<Self> {
        let raw = dialoguer::Password::with_theme(theme)
            .with_prompt("Enter master password")
            .interact()?;

        Ok(Self { raw })
    }

    #[inline]
    fn argon() -> KyResult<Argon2<'static>> {
        let pll = num_cpus::get() as u32;

        // TODO: first arg can be replace by a key file
        let argon = Argon2::new(None, ITR, MEM, pll, Version::default())
            .map_err(|e| KyError::Any(e.to_string()))?;

        Ok(argon)
    }

    pub fn hash(&self) -> KyResult<Encrypted> {
        let salt = SaltString::generate(&mut OsRng);

        let argon = Self::argon()?;

        let hash = argon
            .hash_password_simple(self.raw.as_bytes(), salt.as_ref())
            .map_err(|_| KyError::PwdHash)?
            .to_string();

        Ok(Encrypted::from(hash))
    }

    pub fn verify(&self, hash: &str) -> KyResult<bool> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| KyError::PwdVerify)?;

        let argon = Self::argon()?;

        let is_verified = argon
            .verify_password(self.raw.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_verified)
    }
}

impl AsRef<str> for Master {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}
