use crate::db::DbAction;
use std::io;

pub fn print_menu() {
    println!("Options: ");
    println!("      ");

    let actions = [DbAction::Read, DbAction::AddItem];

    for action in actions {
        match action {
            DbAction::Read => println!("Read"),
            DbAction::AddItem => println!("Add"),
        }
    }
}

pub fn get_user_option() -> DbAction {
    let mut user_input = String::from("");
    if let Err(err) = io::stdin().read_line(&mut user_input) {
        panic!("Wrong option: {}", err);
    };
    match user_input.parse::<DbAction>() {
        Ok(DbAction::Read) => println!("WILL READ"),
        Ok(DbAction::AddItem) => println!("WILL ADD"),
        _ => panic!("Invalid option"),
    }

    println!("CHOICE: {}", user_input);
    DbAction::Read
}
