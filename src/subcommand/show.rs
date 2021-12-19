use super::Command;
use crate::{
    check_db,
    cli::Config,
    lib::{
        entity::{Master, Password},
        Cipher, Decrypted, Encrypted, EntryKey, KyEnv, KyError, KyResult, KyTable, Prompt, Qr,
        MASTER,
    },
};
use clap::Parser;
use tabled::{Alignment, Disable, Full, Indent, Modify, Style, Table, Tabled};

#[derive(Tabled)]
struct Tr(&'static str, String);

#[derive(Debug, Parser)]
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

        let master_cipher = Cipher::for_master(&master_pwd);
        let enc_key = master_cipher.encrypt(&Decrypted::from(&self.key))?;

        // The crypted data returned from database
        // Will be in this format password:username:website:expires:notes
        let encrypted = pwd_db.get(&rtxn, &enc_key)?;

        rtxn.commit()?;

        env.close();

        let key_master = Cipher::for_key(&master_pwd, &self.key)?;

        let val = Password::decrypt(&key_master, &encrypted)?;

        // We can use threads to decrypt each of them
        // and later use .join() to grab the decrypted value
        // Which will make this decryption way faster
        // I tried and I failed, maybe next time

        if self.qr_code {
            let code = Qr::new(&val.password)?.render();
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
                if self.clear {
                    val.password
                } else {
                    "*".repeat(15)
                },
            ),
            Tr("Website", val.website),
            Tr("Expires", val.expires),
            Tr("Notes", val.notes),
        ];

        let table = Table::new(&decrypted)
            .with(Disable::Row(..1))
            .with(Style::pseudo_clean().header(None))
            .with(
                Modify::new(Full)
                    .with(Alignment::left())
                    .with(Indent::new(1, 1, 0, 0)),
            );

        // Don't println! because last line of table already contains a line feed
        print!("{}", table);

        Ok(())
    }
}
