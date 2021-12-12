use super::{entity::Master, Decrypted, Encrypted, EntryKey, KyError, KyResult};
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
    fn make_key<const T: usize>(master: &[u8], key: &[u8], data: &[u8]) -> KyResult<[u8; T]> {
        let h = Hkdf::<Sha256>::new(Some(key), master);
        let mut okm = [0u8; T];
        h.expand(data, &mut okm)
            .map_err(|e| KyError::Any(e.to_string()))?;
        Ok(okm)
    }

    pub fn for_key(master: &Master) -> Self {
        let master_sha = Sha256::digest(master.as_ref().as_bytes());
        let master_key = Key::from_slice(&master_sha);
        let cipher = Aes256GcmSiv::new(master_key);

        let nonce_secret: Vec<u8> = master_key.into_iter().copied().take(12).collect();

        let nonce = Nonce::from_slice(&nonce_secret).to_owned();

        Self { cipher, nonce }
    }

    pub fn for_value(master: &Master, key: &EntryKey) -> KyResult<Self> {
        let master_bytes = master.as_ref().as_bytes();
        let key_bytes = key.as_ref().as_bytes();

        let first_pass = Self::make_key::<256>(master_bytes, key_bytes, &[])?;
        let second_pass = Self::make_key::<256>(master_bytes, key_bytes, &first_pass)?;
        let third_pass = Self::make_key::<32>(&first_pass, &second_pass, master_bytes)?;

        let master_key = Key::from_slice(&third_pass);
        let cipher = Aes256GcmSiv::new(master_key);

        let nonce_secret = Self::make_key::<12>(&second_pass, &first_pass, key_bytes)?;

        let nonce = Nonce::from_slice(&nonce_secret).to_owned();

        Ok(Self { cipher, nonce })
    }

    pub fn encrypt(&self, data: &Decrypted) -> KyResult<Encrypted> {
        let cipher_txt = self
            .cipher
            .encrypt(&self.nonce, data.as_ref().as_bytes())
            .map_err(|_| KyError::Encrypt)?;

        let encrypted = hex::encode(&cipher_txt);

        Ok(Encrypted::from(encrypted))
    }

    pub fn decrypt(&self, encrypted: &Encrypted) -> KyResult<Decrypted> {
        let slice = hex::decode(encrypted.as_ref()).map_err(|_| KyError::Decrypt)?;

        let decrypted = self
            .cipher
            .decrypt(&self.nonce, &slice as &[u8])
            .map_err(|_| KyError::Decrypt)?;

        let decrypted = String::from_utf8(decrypted).map_err(|_| KyError::Decrypt)?;

        Ok(Decrypted::from(decrypted))
    }
}
