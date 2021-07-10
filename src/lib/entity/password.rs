use crate::cli::PasswordParams;

use rand::{thread_rng, Rng};

const CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~`-_+=><.,:;'\"[]{}?/\\|";

#[derive(Debug)]
pub struct Password;

impl Password {
    pub fn generate(opts: &PasswordParams) -> String {
        let charset: Vec<u8> = match &opts.exclude {
            Some(x) => {
                let exclude_bytes = x.as_bytes();

                CHARSET
                    .to_vec()
                    .into_iter()
                    .filter(|c| !exclude_bytes.contains(c))
                    .collect()
            }
            _ => CHARSET.to_vec(),
        };

        let mut rng = thread_rng();

        let len = charset.len();

        let pwd: String = (0..opts.length)
            .map(|_| {
                let idx = rng.gen_range(0..len);
                charset[idx] as char
            })
            .collect();

        pwd
    }
}
