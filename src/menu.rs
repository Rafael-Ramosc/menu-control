pub mod user;
use std::fmt::Display;

use crate::menu::user::User;

pub fn select_menu() {
    println!("Select a option:");
    println!("1. Create new user");
    println!("2. About ME");
    println!("3. EXIT");
}

pub fn select_option() -> i32 {
    println!("Enter a option number:");
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap();
    option.trim().parse::<i32>().unwrap()
}

pub fn option_control(option: i32) {
    match option {
        1 => {
            let user = create_user();
            let user_name = User::get_username(user);
            println!(" \\\\\\ User {} Created! ///", red(user_name));
        }
        2 => println!("My name is Rafael Ramos"),
        3 => println!("Exiting..."),
        _ => println!("Invalid option!"),
    }
}

fn create_user() -> User {
    println!(" ------- Creating new user -------");

    println!("Enter your user name: ");
    let mut user_name_select = String::new();
    std::io::stdin().read_line(&mut user_name_select).unwrap();
    let user_name_select = user_name_select.trim().to_string();

    User::new(0, user_name_select)
}

pub fn bold<T: Display>(text: T) -> String {
    format!("\x1b[1m{}\x1b[0m", text)
}

pub fn color<T: Display>(text: T, color_code: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", color_code, text)
}

pub fn red<T: Display>(text: T) -> String {
    color(text, "31")
}
