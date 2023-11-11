#[derive(Debug)]
struct Person {
    name: String,
}
impl Person {
    fn new(name: String) -> Self {
        Self { name }
    }
}

fn main() {
    let m = String::from("meo");
    let p: Person = Person::new(m);
    println!("IS: {:?}", p);
}
