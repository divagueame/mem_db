use crate::flow;
use crate::person::Person;
use std::process;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Sequence)]
pub enum ItemType {
    Person,
}
impl FromStr for ItemType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "person" => Ok(ItemType::Person),
            _ => Err("Can't parse this Action"),
        }
    }
}

pub struct Db {
    filepath: String,
}

#[derive(Debug)]
pub enum ActionType {
    Read,
    AddItem,
    CloseConnection,
}

#[derive(Debug)]
pub enum Action<T: Databaseable> {
    AddItem(T),
    Read,
    CloseConnection,
}

pub trait Databaseable {
    fn parse(&self) -> String;
    fn build_from_user() -> Self;
}

impl<T: Databaseable> Action<T> {
    pub fn execute(&self, db: &mut Db) {
        match self {
            Action::Read => {
                println!("READING: {:?}", db.read());
                ()
            }
            Action::AddItem(item) => {
                println!("Execute Add items!, {:?}", item.parse());
                let res = db.add(item).unwrap();
                res
            }
            Action::CloseConnection => {
                println!("Execute exit!");
                process::exit(1);
            }
        }
    }
}

impl FromStr for ActionType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "read" => Ok(ActionType::Read),
            "add" => Ok(ActionType::AddItem),
            "exit" => Ok(ActionType::CloseConnection),
            _ => Err("Can't parse this Action"),
        }
    }
}

impl Db {
    pub fn new(filepath: &str) -> Self {
        println!("filepath {}", filepath);
        Self {
            filepath: filepath.to_string(),
        }
    }

    pub fn add<T>(&mut self, item: &T) -> Result<(), Box<dyn Error>>
    where
        T: Databaseable,
    {
        let mut f = File::options().append(true).open(&self.filepath)?;

        println!("WRITING!{:}", item.parse());
        writeln!(&mut f, "{}", item.parse())?;
        Ok(())
    }

    pub fn read(&self) -> Vec<Person> {
        let mut entries: Vec<Person> = vec![];
        let f = File::open(&self.filepath).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_db() -> Db {
        fs::write("./db_test.txt", "").unwrap();
        let db_file = String::from("./db_test.txt");

        Db::new(&db_file)
    }

    #[test]
    // #[ignore]
    fn creates_db() {
        let db = setup_db();

        assert_eq!(db.filepath, String::from("./db_test.txt"));
        assert!(matches!(db, Db { .. }));
    }

    #[test]
    // #[ignore]
    fn on_start_read_items() {
        let db = setup_db();
        let entries = db.read();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    // #[ignore]
    fn add_item() {
        let mut db = setup_db();
        let pre_entries = db.read();

        let name = String::from("Mike");
        let email = String::from("miki@miki.com");
        let new_item = Person::new(name, email);

        let res = db.add(&new_item);
        assert!(matches!(res, Ok(())));

        let after_entries = db.read();
        assert_ne!(pre_entries.len(), after_entries.len());
    }
}
