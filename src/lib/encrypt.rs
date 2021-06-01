use aes_gcm_siv::{
    aead::{Aead, NewAead},
    Aes256GcmSiv, Key, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use sha2::{digest::Output, Digest, Sha256};

use super::KyError;

pub struct Encrypt<'a> {
    key: &'a str,
    data: &'a str,
}

// TODO: handle .unwrap()
impl<'a> Encrypt<'a> {
    pub fn new(key: &'a str, data: &'a str) -> Self {
        Self { key, data }
    }

    pub fn get_sha(&self) -> Output<Sha256> {
        let mut sha_256 = Sha256::new();
        sha_256.update(self.key.as_bytes());

        sha_256.finalize()
    }

    pub fn encrypt(&self) -> Result<(String, String), KyError> {
        let key_sha = self.get_sha();
        let key = Key::from_slice(&key_sha);
        let cipher = Aes256GcmSiv::new(key);

        let nonce = {
            let mut v = [0u8; 12];
            OsRng.fill_bytes(&mut v);
            v
        };

        let nonce = Nonce::from_slice(&nonce);

        let cipher_txt = cipher.encrypt(nonce, self.data.as_bytes()).unwrap();
        let cipher_nonce = cipher.encrypt(nonce, nonce.as_ref()).unwrap();

        let txt_utf = String::from_utf8_lossy(&cipher_txt).to_string();
        let nonce_utf = String::from_utf8_lossy(&cipher_nonce).to_string();

        Ok((txt_utf, nonce_utf))
    }
}
