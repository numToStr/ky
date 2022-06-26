use super::{
    entity::{Master, Password},
    Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable,
};
use crate::lib::Cipher;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fs::File, io::Cursor, path::Path};

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

pub struct Vault;

impl Vault {
    /// Create database backup as tarball w/ zstd compression
    pub fn backup(from: &Path, to: &Path) -> KyResult<()> {
        let mut data = {
            let mut tar = tar::Builder::new(Vec::with_capacity(1));
            tar.append_dir_all(".", &from)?;
            Cursor::new(tar.into_inner()?)
        };

        let mut encoder = {
            let target = File::create(to)?;
            zstd::Encoder::new(target, 5)?
        };

        std::io::copy(&mut data, &mut encoder)?;

        encoder.finish()?;

        Ok(())
    }

    /// Restore database backup
    pub fn restore(from: &Path, to: &Path) -> KyResult<()> {
        let decoder = zstd::Decoder::new(File::open(from)?)?;
        let mut tar = tar::Archive::new(decoder);
        tar.unpack(&to)?;

        Ok(())
    }

    pub fn export(
        dest: &Path,
        master: &Master,
        entries: Vec<(Encrypted, Encrypted)>,
    ) -> KyResult<()> {
        let mut wtr = Writer::from_path(dest).map_err(|_| KyError::ExportCreate)?;
        let master_cipher = Cipher::from(master);

        for (k, v) in entries.into_iter() {
            let key = master_cipher.decrypt(&k)?.into();
            let key_cipher = Cipher::try_from((master, &key))?;
            let val = Password::decrypt(&key_cipher, &v)?;
            let key_ref = key.as_ref().to_string();

            wtr.serialize(Row {
                title: key,
                website: val.website,
                username: val.username,
                password: val.password,
                notes: val.note,
                expires: val.expires,
            })
            .map_err(|_| KyError::Export(key_ref))?;
        }

        Ok(())
    }

    pub fn import(src: &Path, master: &Master, env: &KyEnv) -> KyResult<()> {
        let mut rdr = Reader::from_path(src).map_err(|_| KyError::ImportRead)?;
        let iter = rdr.deserialize();

        let common_db = env.get_table(KyTable::Master)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let mut wtxn = env.write_txn()?;

        let hashed = master.hash()?;

        common_db.set(&mut wtxn, &Encrypted::from(Master::KEY), &hashed)?;

        let master_cipher = Cipher::from(master);

        for (i, entry) in iter.enumerate() {
            let k: Row = entry.map_err(|_| KyError::Import(i))?;

            let key_cipher = Cipher::try_from((master, &k.title))?;

            let val = Password {
                username: k.username,
                password: k.password,
                website: k.website,
                expires: k.expires,
                note: k.notes,
            }
            .encrypt(&key_cipher)?;

            let key = master_cipher.encrypt(&k.title.into())?;

            pwd_db.set(&mut wtxn, &key, &val)?;
        }

        wtxn.commit()?;

        Ok(())
    }
}
