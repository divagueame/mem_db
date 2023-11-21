use crate::db::{Action, ActionType};
use std::io;

pub fn print_menu() {
    println!("Options: ");
    println!("      ");

    let action_types = [
        ActionType::Read,
        ActionType::AddItem,
        ActionType::CloseConnection,
    ];

    for action in action_types {
        match action {
            ActionType::Read => println!("read => Read database"),
            ActionType::AddItem => println!("add => Add item to database"),
            ActionType::CloseConnection => println!("exit => Exit"),
        }
    }
}

pub fn get_user_action_type() -> ActionType {
    let mut user_input = String::from("");
    if let Err(err) = io::stdin().read_line(&mut user_input) {
        panic!("Wrong option: {}", err);
    };

    match user_input.parse::<ActionType>() {
        Ok(ActionType::Read) => ActionType::Read,
        Ok(ActionType::AddItem) => ActionType::AddItem,
        Ok(ActionType::CloseConnection) => ActionType::CloseConnection,
        _ => ActionType::CloseConnection,
    }
}
