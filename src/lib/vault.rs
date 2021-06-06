use super::KyError;
use std::{fs::File, path::Path};

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
}
