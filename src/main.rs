pub mod db;
mod flow;
#[warn(unused_variables, unused_imports)]
mod person;

use db::{Action, ActionType, Databaseable, Db};
use person::Person;

fn main() {
    let db_filepath = String::from("./db.txt");
    let mut db = Db::new(&db_filepath);

    loop {
        flow::print_menu();
        let user_action_type = flow::get_user_action_type();
        match user_action_type {
            ActionType::Read => println!("READ"),
            ActionType::AddItem => {
                let item = Person::build_from_user();
                Action::AddItem(item).execute(&mut db);
            }
            ActionType::CloseConnection => println!("Bye!"),
        }
    }
}
