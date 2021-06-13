use std::fmt::{self, Display, Formatter};

pub const DELIM: char = ':';
pub const EMPTY: &str = "-";

#[macro_use]
macro_rules! create_key {
    ($k: expr) => {{
        match $k.next() {
            Some("") => None,
            Some(x) => Some(x.to_string()),
            _ => None,
        }
    }};
}

type Val = Option<String>;

#[derive(Debug)]
pub struct Values {
    pub password: Val,
    pub username: Val,
    pub website: Val,
    pub expires: Val,
    pub notes: Val,
}

impl From<&str> for Values {
    fn from(s: &str) -> Self {
        let mut keys = s.splitn(5, DELIM);

        let password = create_key!(keys);
        let username = create_key!(keys);
        let website = create_key!(keys);
        let expires = create_key!(keys);
        let notes = create_key!(keys);

        Self {
            password,
            username,
            website,
            expires,
            notes,
        }
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{d}{}{d}{}{d}{}{d}{}",
            self.password.as_deref().unwrap_or_default(),
            self.username.as_deref().unwrap_or_default(),
            self.website.as_deref().unwrap_or_default(),
            self.expires.as_deref().unwrap_or_default(),
            self.notes.as_deref().unwrap_or_default(),
            d = DELIM,
        )
    }
}
