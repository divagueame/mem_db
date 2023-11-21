pub mod db;
mod flow;
#[warn(unused_variables, unused_imports)]
mod person;

use db::Db;

fn main() {
    let db_filepath = String::from("./db.txt");
    let mut db = Db::new(&db_filepath);

    loop {
        flow::print_menu();
        let user_action = flow::get_user_action_type();
        user_action.execute(&mut db);
    }
}
