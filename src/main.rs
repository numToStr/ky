mod lib;

use lib::Database;

fn main() {
    let db = Database::new("lok.db").unwrap();
    let key = "hello";
    db.set(key, "world").unwrap();
    println!("{:#?}", db.get(key));
}
