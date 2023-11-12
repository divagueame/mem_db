#[warn(unused_variables)]
use person::Person;
mod person;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let db = read_db();
    println!("{:?}", db);
}

fn read_db() -> Vec<Person> {
    let mut entries: Vec<Person> = vec![];
    let f = File::open("./db.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut values = line.split("|");
            let mut name = String::new();
            let mut email = String::new();

            if let Some(name_entry) = values.next() {
                name = name_entry.to_string();
            };
            if let Some(email_entry) = values.next() {
                email = email_entry.to_string();
            }
            let person_entry = Person::new(name, email);
            entries.push(person_entry)
        };
    }
    entries
}
