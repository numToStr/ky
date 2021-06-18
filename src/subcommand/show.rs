use super::Command;
use crate::{
    check_db,
    cli::Config,
    lib::{Cipher, Database, KyError, Password, Prompt, Qr, Value, MASTER},
};
use clap::Clap;
use tabled::{table, Alignment, Disable, Full, Indent, Row, Style, Tabled};

#[derive(Tabled)]
struct Detail(&'static str, String);

#[derive(Debug, Clap)]
pub struct Show {
    /// Entry which need to be shown
    key: String,

    /// Show password in clear text
    #[clap(short = 'C', long)]
    clear: bool,

    /// Show password in a form of qr code
    #[clap(short, long)]
    qr_code: bool,

    /// Don't print the details, can be used with qr code
    #[clap(short, long, conflicts_with = "clear")]
    mute: bool,
}

impl Command for Show {
    fn exec(&self, config: Config) -> Result<(), KyError> {
        let db_path = config.db_path();

        check_db!(db_path);

        let master_pwd = Password::ask_master(&Prompt::theme())?;

        let db = Database::open(&db_path)?;

        let rtxn = db.read_txn()?;
        let hashed = db.get(&rtxn, &MASTER)?;

        if !master_pwd.verify(&hashed)? {
            return Err(KyError::MisMatch);
        }

        // The crypted data returned from database
        // Will be in this format password:username:website:expires:notes
        let encrypted = db.get(&rtxn, &self.key)?;

        rtxn.commit()?;

        db.close();

        let cipher = Cipher::new(&master_pwd.to_string(), &self.key);

        let val = Value::decrypt(&cipher, &encrypted)?;

        // We can use threads to decrypt each of them
        // and later use .join() to grab the decrypted value
        // Which will make this decryption way faster
        // I tried and I failed, maybe next time

        let password = if self.clear || self.qr_code {
            Some(cipher.decrypt(&val.password)?)
        } else {
            None
        };

        if let (true, Some(p)) = (self.qr_code, &password) {
            let code = Qr::new(&p)?.render();
            eprint!("{}", code);
        }

        // If the output is muted then no need to print the table
        if self.mute {
            return Ok(());
        }

        let decrypted = [
            Detail("Username", val.username),
            Detail(
                "Password",
                if let (true, Some(p)) = (self.clear, password) {
                    p
                } else {
                    "*".repeat(15)
                },
            ),
            Detail("Website", val.website),
            Detail("Expires", val.expires),
            Detail("Notes", val.notes),
        ];

        let table = table!(
            &decrypted,
            Disable::Row(..1),
            Style::pseudo_clean().header(None),
            Alignment::left(Full),
            Indent::new(Row(..), 1, 1, 0, 0)
        );

        // Don't println! because last line of table already contains a line feed
        print!("{}", table);

        Ok(())
    }
}
