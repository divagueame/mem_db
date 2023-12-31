use crate::db::{Action, ActionType, Databaseable, ItemType};
use crate::person::Person;
use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
use std::{io, process};

pub fn print_menu() {
    println!("Options: ");
    println!("      ");

    let action_types = [
        ActionType::Read,
        ActionType::AddItem,
        ActionType::CloseConnection,
    ];

    for action_type in action_types {
        match action_type {
            ActionType::Read => println!("read => Read database"),
            ActionType::AddItem => println!("add => Add item to database"),
            ActionType::CloseConnection => println!("exit => Exit"),
        }
    }
}

pub fn get_user_action_type() -> ActionType {
    println!("----");
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

pub fn get_user_item_type() -> ItemType {
    println!("=== Type the type to add ===");
    for v in all::<ItemType>() {
        println!("{:?}", v);
    }
    println!("----");
    let mut user_input = String::from("");
    if let Err(err) = io::stdin().read_line(&mut user_input) {
        panic!("Wrong option: {}", err);
    };

    user_input.parse::<ItemType>().unwrap()
}
