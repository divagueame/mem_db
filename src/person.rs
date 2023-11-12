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
