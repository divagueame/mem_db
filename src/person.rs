#[derive(Debug)]
pub struct Person {
    pub name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
