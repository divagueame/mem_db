#[warn(unused_variables)]
use person::Person;
mod person;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let db = read_db();
    // println!("{:?}", db);
}

fn read_db() -> Vec<Person> {
    let f = File::open("./db.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::from("LINE");
    reader.read_line(&mut line).unwrap();
    println!("IS: {:?}", line);
    reader.read_line(&mut line).unwrap();
    println!("IS: {:?}", line);

    let m = String::from("Chiki");
    let p: Person = Person::new(m);

    vec![p]
}
