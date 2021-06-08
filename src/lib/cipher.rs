use super::KyError;
use aes_gcm_siv::{
    aead::{Aead, NewAead},
    Aes256GcmSiv, Key, Nonce,
};
use sha2::{digest::Output, Digest, Sha256};

pub struct Cipher {
    cipher: Aes256GcmSiv,
    nonce: Nonce,
}

impl Cipher {
    pub fn new(master: &str, key: &str) -> Self {
        let master_sha = Self::get_sha(master);
        let master_key = Key::from_slice(&master_sha);
        let cipher = Aes256GcmSiv::new(master_key);

        let iter_count = 6;
        let nonce_secret: Vec<u8> = Self::get_sha(key)
            .into_iter()
            .take(iter_count)
            .chain(master_key.into_iter().copied().take(iter_count))
            .collect();

        let nonce = Nonce::from_slice(&nonce_secret).to_owned();

        Self { cipher, nonce }
    }

    #[inline]
    fn get_sha(key: &str) -> Output<Sha256> {
        Sha256::digest(key.as_bytes())
    }

    pub fn encrypt(&self, data: &str) -> Result<String, KyError> {
        let cipher_txt = self
            .cipher
            .encrypt(&self.nonce, data.as_bytes())
            .map_err(|_| KyError::Encrypt)?;

        let pwd_encrypted = hex::encode(&cipher_txt);

        Ok(pwd_encrypted)
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String, KyError> {
        let slice = hex::decode(encrypted).map_err(|_| KyError::Decrypt)?;

        let decrypted = self
            .cipher
            .decrypt(&self.nonce, &slice as &[u8])
            .map_err(|_| KyError::Decrypt)?;

        let pwd_decrypted = String::from_utf8(decrypted).map_err(|_| KyError::Decrypt)?;

        Ok(pwd_decrypted)
    }
}
