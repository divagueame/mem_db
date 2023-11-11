use person::Person;
mod person;

fn main() {
    let m = String::from("meo");
    let p: Person = Person::new(m);
    println!("IS: {:?}", p.name);
}
