use crate::person::Person;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
pub struct Db {}

impl Db {
    pub fn new(filename: &str) -> Self {
        println!("Filename {}", filename);
        Self {}
    }

    pub fn read(&self) -> Vec<Person> {
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
}