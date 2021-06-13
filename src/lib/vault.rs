use super::{KyError, Values};
use crate::{check_decrypt, lib::Cipher};
use csv::Writer;
use serde::Serialize;
use std::{fs::File, path::Path};

/// Row represent a csv row.
/// NOTE: don't change the sequence of fields because it's same as 1Password csv export
#[derive(Debug, Serialize)]
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
        master_pwd: &str,
        entries: Vec<(String, String)>,
        dest: &'a Path,
    ) -> Result<(), KyError> {
        let mut wtr = Writer::from_path(dest).unwrap();

        for entry in entries.into_iter() {
            let k = entry.0;
            let cipher = Cipher::new(master_pwd, &k);

            let val = Values::from(entry.1.as_str());

            wtr.serialize(Row {
                title: k,
                website: check_decrypt!(&cipher, &val.url),
                username: check_decrypt!(&cipher, &val.username),
                password: check_decrypt!(&cipher, &val.password),
                notes: check_decrypt!(&cipher, &val.notes),
                expires: check_decrypt!(&cipher, &val.expires),
            })
            .map_err(|x| KyError::Any(x.to_string()))?;
        }

        Ok(())
    }
}
