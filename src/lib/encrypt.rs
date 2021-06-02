use super::KyError;
use aes_gcm_siv::{
    aead::{Aead, NewAead},
    Aes256GcmSiv, Key, Nonce,
};
use sha2::{digest::Output, Digest, Sha256};

pub struct Encrypt<'a> {
    key: &'a str,
}

impl<'a> Encrypt<'a> {
    pub fn new(key: &'a str) -> Self {
        Self { key }
    }

    fn get_sha(&self) -> Output<Sha256> {
        Sha256::digest(self.key.as_bytes())
    }

    pub fn encrypt(&self, data: &str) -> Result<String, KyError> {
        let key_sha = self.get_sha();
        let key = Key::from_slice(&key_sha);
        let cipher = Aes256GcmSiv::new(key);

        let nonce_secret: Vec<u8> = key_sha.into_iter().take(12).collect();
        let nonce = Nonce::from_slice(&nonce_secret);

        let cipher_txt = cipher
            .encrypt(nonce, data.as_bytes())
            .map_err(|_| KyError::Encrypt)?;

        let pwd_encrypted = hex::encode(&cipher_txt);

        Ok(pwd_encrypted)
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String, KyError> {
        let key_sha = self.get_sha();
        let key = Key::from_slice(&key_sha);
        let cipher = Aes256GcmSiv::new(key);

        let nonce_secret: Vec<u8> = key_sha.into_iter().take(12).collect();
        let nonce = Nonce::from_slice(&nonce_secret);

        let slice = hex::decode(encrypted).map_err(|_| KyError::Decrypt)?;

        let decrypted = cipher
            .decrypt(nonce, &slice as &[u8])
            .map_err(|_| KyError::Decrypt)?;

        let pwd_decrypted = String::from_utf8(decrypted).map_err(|_| KyError::Decrypt)?;

        Ok(pwd_decrypted)
    }
}
