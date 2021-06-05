use std::fmt::{self, Display, Formatter};

pub const DELIM: char = ':';
pub const EMPTY: &str = "-";

#[macro_use]
macro_rules! create_key {
    ($k: expr) => {
        $k.next().unwrap_or(EMPTY).to_string()
    };
}

pub struct Keys {
    pub password: String,
    pub username: String,
    pub url: String,
    pub expires: String,
    pub notes: String,
}

pub struct Value {
    pub keys: Keys,
}

impl Value {
    pub fn new(keys: Keys) -> Self {
        Self { keys }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        let mut keys = s.splitn(5, DELIM);

        let password = create_key!(keys);
        let username = create_key!(keys);
        let url = create_key!(keys);
        let expires = create_key!(keys);
        let notes = create_key!(keys);

        Self::new(Keys {
            password,
            username,
            url,
            expires,
            notes,
        })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{d}{}{d}{}{d}{}{d}{}",
            self.keys.password,
            self.keys.username,
            self.keys.url,
            self.keys.expires,
            self.keys.notes,
            d = DELIM,
        )
    }
}
