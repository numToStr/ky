use super::{
    entity::{Master, Password},
    Decrypted, Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable, MASTER,
};
use crate::lib::Cipher;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

/// Row represent a csv row.
/// NOTE: don't change the sequence of fields because it's same as 1Password csv export
#[derive(Debug, Serialize, Deserialize)]
struct Row {
    title: EntryKey,
    website: String,
    username: String,
    password: String,
    notes: String,
    expires: String,
}

pub struct Vault<'a> {
    src: &'a Path,
}

impl<'a> Vault<'a> {
    pub fn new(src: &'a Path) -> Self {
        Self { src }
    }

    pub fn backup(&self, dest: &'a Path) -> KyResult<File> {
        let mut tar = tar::Builder::new(Vec::new());

        tar.append_dir_all(".", self.src)?;

        let tarred = tar.into_inner()?;

        let mut data = std::io::Cursor::new(tarred);

        let mut encoder = {
            let target = std::fs::File::create(dest)?;
            zstd::Encoder::new(target, 5)?
        };

        std::io::copy(&mut data, &mut encoder)?;

        let f = encoder.finish()?;

        Ok(f)
    }

    pub fn restore(&self, dest: &'a Path) -> KyResult<()> {
        let decoder = {
            let file = File::open(self.src)?;
            zstd::Decoder::new(file)?
        };

        let mut tar = tar::Archive::new(decoder);

        tar.unpack(dest)?;

        Ok(())
    }

    pub fn export(
        dest: &'a Path,
        master: &Master,
        entries: Vec<(Encrypted, Encrypted)>,
    ) -> KyResult<()> {
        let mut wtr = Writer::from_path(dest).map_err(|_| KyError::ExportCreate)?;
        let master_cipher = Cipher::for_master(master);

        for (k, v) in entries.into_iter() {
            let key = master_cipher.decrypt(&k)?.into();
            let key_cipher = Cipher::for_key(master, &key)?;
            let val = Password::decrypt(&key_cipher, &v)?;
            let key_ref = key.as_ref().to_string();

            wtr.serialize(Row {
                title: key,
                website: val.website,
                username: val.username,
                password: val.password,
                notes: val.notes,
                expires: val.expires,
            })
            .map_err(|_| KyError::Export(key_ref))?;
        }

        Ok(())
    }

    pub fn import(src: &Path, master: &Master, env: &KyEnv) -> KyResult<()> {
        let mut rdr = Reader::from_path(src).map_err(|_| KyError::ImportRead)?;
        let iter = rdr.deserialize();

        let common_db = env.get_table(KyTable::Common)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let mut wtxn = env.write_txn()?;

        let hashed = master.hash()?;

        common_db.set(&mut wtxn, &Encrypted::from(MASTER), &hashed)?;

        let master_cipher = Cipher::for_master(master);

        for (i, entry) in iter.enumerate() {
            let k: Row = entry.map_err(|_| KyError::Import(i))?;

            let key_cipher = Cipher::for_key(master, &k.title)?;

            let val = Password {
                username: k.username,
                password: k.password,
                website: k.website,
                expires: k.expires,
                notes: k.notes,
            }
            .encrypt(&key_cipher)?;

            let key = master_cipher.encrypt(&Decrypted::from(&k.title))?;

            pwd_db.set(&mut wtxn, &key, &val)?;
        }

        wtxn.commit()?;

        Ok(())
    }
}
