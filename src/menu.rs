pub mod helpers;
pub mod json;
pub mod user;

use crate::menu::helpers::*;
use crate::menu::user::{Configuration, User};
use colored::*;
use std::process;

pub fn select_menu() {
    println!("Select a option:");
    println!("1. Create new user");
    println!("2. List of users");
    println!("3. User preferences");
    println!("4. About ME");
    println!("{}", "5. EXIT".red());
}

pub fn select_option() -> i32 {
    println!("Enter a option number:");
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap();

    let option = option.trim();

    match option.parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

pub fn option_control(option: i32) {
    match option {
        1 => {
            let user = create_user();
            let user_name = User::get_username(user);
            println!(" User {} Created!", user_name.red());
        }
        2 => {
            println!("List of users:");
            users_list()
        }
        3 => users_list(),
        4 => println!("Rafael Ramos - rafael.ramosrc@gmail.com"),
        5 => {
            println!("Closing...");
            process::exit(0);
        }
        _ => println!("Invalid option!"),
    }
}
