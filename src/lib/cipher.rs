use super::{KyError, Password};
use aes_gcm_siv::{
    aead::{Aead, NewAead},
    Aes256GcmSiv, Key, Nonce,
};
use hkdf::Hkdf;
use sha2::{Digest, Sha256};

pub struct Cipher {
    cipher: Aes256GcmSiv,
    nonce: Nonce,
}

impl Cipher {
    #[inline]
    fn make_key<const T: usize>(
        master: &[u8],
        key: &[u8],
        data: &[u8],
    ) -> Result<[u8; T], KyError> {
        let h = Hkdf::<Sha256>::new(Some(key), master);
        let mut okm = [0u8; T];
        h.expand(data, &mut okm)
            .map_err(|e| KyError::Any(e.to_string()))?;
        Ok(okm)
    }

    pub fn for_key(master: &Password) -> Self {
        let master_sha = Sha256::digest(master.to_string().as_bytes());
        let master_key = Key::from_slice(&master_sha);
        let cipher = Aes256GcmSiv::new(master_key);

        let nonce_secret: Vec<u8> = master_key.into_iter().copied().take(12).collect();

        let nonce = Nonce::from_slice(&nonce_secret).to_owned();

        Self { cipher, nonce }
    }

    pub fn for_value(master: &Password, key: &str) -> Result<Self, KyError> {
        let m = master.to_string();
        let master_bytes = m.as_bytes();
        let key_bytes = key.as_bytes();

        let first_pass = Self::make_key::<256>(master_bytes, key_bytes, &[])?;
        let second_pass = Self::make_key::<256>(master_bytes, key_bytes, &first_pass)?;
        let third_pass = Self::make_key::<32>(&first_pass, &second_pass, master_bytes)?;

        let master_key = Key::from_slice(&third_pass);
        let cipher = Aes256GcmSiv::new(master_key);

        let nonce_secret = Self::make_key::<12>(&second_pass, &first_pass, &key_bytes)?;

        let nonce = Nonce::from_slice(&nonce_secret).to_owned();

        Ok(Self { cipher, nonce })
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
