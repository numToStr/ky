use std::convert::TryFrom;

use super::Command;
use crate::{
    cli::Config,
    lib::{
        entity::{Master, Password},
        Cipher, EntryKey, KyDb2, KyError, KyResult, KyTable, Prompt, Qr,
    },
};
use clap::Parser;
use tabled::{object::Segment, Alignment, Disable, Modify, Style, Table, Tabled};

#[derive(Tabled)]
struct Tr(&'static str, String);

#[derive(Debug, Parser)]
pub struct Show {
    /// Entry which need to be shown
    key: EntryKey,

    /// Show password in clear text
    #[clap(short, long)]
    clear: bool,

    /// Show password in a form of qr code
    #[clap(short, long)]
    qr_code: bool,

    /// Don't print the details, can be used with qr code
    #[clap(short, long, conflicts_with = "clear")]
    mute: bool,
}

impl Command for Show {
    fn exec(self, config: Config) -> KyResult<()> {
        let master = Master::ask(&Prompt::theme())?;

        let db = KyDb2::connect(&config.db_path())?;

        let rtxn = db.rtxn()?;

        {
            let master_tbl = db.open_read(&rtxn, KyTable::Master)?;
            let hashed = master_tbl.get(&Master::KEY.into())?;

            if !master.verify(hashed)? {
                return Err(KyError::MisMatch);
            }
        }

        let data = {
            let enc_key = Cipher::from(&master).encrypt(&self.key.clone().into())?;
            let encrypted = db.open_read(&rtxn, KyTable::Password)?.get(&enc_key)?;
            let decrypted = Cipher::try_from((&master, &self.key))?.decrypt(&encrypted)?;
            Password::try_from(decrypted)?
        };

        if self.qr_code {
            let code = Qr::new(&data.password)?.render();
            eprint!("{code}");
        }

        // If the output is muted then no need to print the table
        if self.mute {
            return Ok(());
        }

        let decrypted = [
            Tr("Username", data.username),
            Tr(
                "Password",
                if self.clear {
                    data.password
                } else {
                    "*".repeat(15)
                },
            ),
            Tr("Website", data.website),
            Tr("Expires", data.expires),
            Tr("Note", data.note),
        ];

        let table = Table::new(&decrypted)
            .with(Disable::Row(..1))
            .with(Style::modern().header_off().horizontal_off())
            .with(Modify::new(Segment::all()).with(Alignment::left()));

        // Don't println! because last line of table already contains a line feed
        print!("{table}");

        Ok(())
    }
}
