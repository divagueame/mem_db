use crate::db::Databaseable;
use core::fmt;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub email: String,
}

impl Person {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}\n", self.name, self.email)
    }
}

impl Databaseable for Person {
    fn parse(&self) -> String {
        format!("{}|{}", self.name, self.email)
    }
}
