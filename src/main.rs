mod db;

use db::Database;

fn main() {
    let dbb = Database::new();
    let key = "hello";
    dbb.set(key, "world");
    println!("{:#?}", dbb.get(key));
}
