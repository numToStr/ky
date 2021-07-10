use super::Command;
use crate::{
    check_db,
    cli::Config,
    lib::{
        entity::Master, Cipher, Decrypted, Details, Encrypted, EntryKey, KyEnv, KyError, KyResult,
        KyTable, Prompt, Qr, MASTER,
    },
};
use clap::Clap;
use tabled::{table, Alignment, Disable, Full, Indent, Row, Style, Tabled};

#[derive(Tabled)]
struct Tr(&'static str, String);

#[derive(Debug, Clap)]
pub struct Show {
    /// Entry which need to be shown
    key: EntryKey,

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
    fn exec(&self, config: Config) -> KyResult<()> {
        let db_path = config.db_path();

        check_db!(db_path);

        let master_pwd = Master::ask(&Prompt::theme())?;

        let env = KyEnv::connect(&db_path)?;

        let common_db = env.get_table(KyTable::Common)?;
        let pwd_db = env.get_table(KyTable::Password)?;

        let rtxn = env.read_txn()?;
        let hashed = common_db.get(&rtxn, &Encrypted::from(MASTER))?;

        if !master_pwd.verify(hashed.as_ref())? {
            return Err(KyError::MisMatch);
        }

        let key_cipher = Cipher::for_key(&master_pwd);
        let key = key_cipher.encrypt(&Decrypted::from(&self.key))?;

        // The crypted data returned from database
        // Will be in this format password:username:website:expires:notes
        let encrypted = pwd_db.get(&rtxn, &key)?;

        rtxn.commit()?;

        env.close();

        let cipher = Cipher::for_value(&master_pwd, &self.key)?;

        let val = Details::decrypt(&cipher, &encrypted)?;

        // We can use threads to decrypt each of them
        // and later use .join() to grab the decrypted value
        // Which will make this decryption way faster
        // I tried and I failed, maybe next time

        let password = if self.clear || self.qr_code {
            Some(cipher.decrypt(&Encrypted::from(val.password))?)
        } else {
            None
        };

        if let (true, Some(p)) = (self.qr_code, &password) {
            let code = Qr::new(&p.as_ref())?.render();
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
                    p.into()
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
