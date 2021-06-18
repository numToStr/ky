use super::Command;
use crate::{
    check_db,
    cli::Config,
    lib::{Cipher, Database, Details, KyError, Password, Prompt, Qr, MASTER},
};
use clap::Clap;
use tabled::{table, Alignment, Disable, Full, Indent, Row, Style, Tabled};

#[derive(Tabled)]
struct Tr(&'static str, String);

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

        let key = Cipher::for_key(&master_pwd).encrypt(&self.key)?;

        // The crypted data returned from database
        // Will be in this format password:username:website:expires:notes
        let encrypted = db.get(&rtxn, &key)?;

        rtxn.commit()?;

        db.close();

        let cipher = Cipher::for_value(&master_pwd, &self.key)?;

        let val = Details::decrypt(&cipher, &encrypted)?;

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
            Tr("Username", val.username),
            Tr(
                "Password",
                if let (true, Some(p)) = (self.clear, password) {
                    p
                } else {
                    "*".repeat(15)
                },
            ),
            Tr("Website", val.website),
            Tr("Expires", val.expires),
            Tr("Notes", val.notes),
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
