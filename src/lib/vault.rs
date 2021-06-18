use super::{Database, KyError, Value, MASTER};
use crate::lib::{Cipher, Password};
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

/// Row represent a csv row.
/// NOTE: don't change the sequence of fields because it's same as 1Password csv export
#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    title: String,
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

    pub fn backup(&self, dest: &'a Path) -> Result<File, KyError> {
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

    pub fn restore(&self, dest: &'a Path) -> Result<(), KyError> {
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
        master_pwd: &str,
        entries: Vec<(String, String)>,
    ) -> Result<(), KyError> {
        let mut wtr = Writer::from_path(dest).map_err(|_| KyError::ExportCreate)?;

        for entry in entries.into_iter() {
            let k = entry.0;
            let cipher = Cipher::new(master_pwd, &k);
            let val = Value::decrypt(&cipher, entry.1.as_str())?;

            wtr.serialize(Row {
                title: k.to_string(),
                website: val.website,
                username: val.username,
                password: cipher.decrypt(&val.password)?,
                notes: val.notes,
                expires: val.expires,
            })
            .map_err(|_| KyError::Export(k))?;
        }

        Ok(())
    }

    pub fn import(src: &Path, master_pwd: Password, db: &Database) -> Result<(), KyError> {
        let mut rdr = Reader::from_path(src).map_err(|_| KyError::ImportRead)?;
        let iter = rdr.deserialize();

        let mut wtxn = db.write_txn()?;

        db.set(&mut wtxn, MASTER, &master_pwd.hash()?)?;

        let pwd = master_pwd.to_string();

        for (i, entry) in iter.enumerate() {
            let k: Row = entry.map_err(|_| KyError::Import(i))?;

            let cipher = Cipher::new(&pwd, &k.title);

            let val = Value {
                username: k.username,
                password: k.password,
                website: k.website,
                expires: k.expires,
                notes: k.notes,
            }
            .encrypt(&cipher)?;

            db.set(&mut wtxn, &k.title, &val.to_string())?;
        }

        wtxn.commit()?;

        Ok(())
    }
}
