mod db;
#[warn(unused_variables, unused_imports)]
mod person;

use db::Db;

fn main() {
    let db_file = String::from("./db.txt");
    let db = Db::new(&db_file);
    let db_entries = db.read();
    println!("{:?}", db_entries);
}
