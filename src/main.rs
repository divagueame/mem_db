pub mod db;
#[warn(unused_variables, unused_imports)]
mod person;

use db::Db;
use person::Person;

fn main() {
    let db_filepath = String::from("./db.txt");
    let mut db = Db::new(&db_filepath);
    let db_entries = db.read();

    println!("Initial entries: {:?}", db_entries.len());
    let name = String::from("Mike");
    let email = String::from("miki@miki.com");
    let p = Person::new(name, email);
    db.add(p);
    println!("Initial entries: {:?}", db_entries.len());
}
