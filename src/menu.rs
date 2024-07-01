pub mod json;
pub mod user;

use colored::*;
use serde_json::Value;
use std::process;

use crate::menu::json::json_read;
use crate::menu::user::{Configuration, User};

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

fn create_user() -> User {
    println!(" ------- Creating new user -------");

    println!("Enter your user name: ");
    let mut user_name_select = String::new();
    std::io::stdin().read_line(&mut user_name_select).unwrap();
    let user_name_select = user_name_select.trim().to_string();

    let user: User = User::new(0, user_name_select);
    let new_user_json =
        serde_json::to_string_pretty(&user).expect("Error converting String to Json");

    json::json_data(&new_user_json).expect("Error when trying to write json");
    println!("{:?}", new_user_json);
    user
}

fn users_list() {
    println!("User list:");
    match get_all_users() {
        Ok(users) => {
            let mut count = 0;
            for user in users {
                println!("{} User: {}", count, user.get_username());
                count += 1;
            }
            println!("TOTAL OF USERS: {}", (count).to_string().yellow());
        }
        Err(e) => {
            println!("Error when trying to read users: {}", e);
        }
    }
}

pub fn get_all_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string("data/user.json")?;
    let json: Value = serde_json::from_str(&file_content)?;

    if let Some(users) = json["users"].as_array() {
        let user_vec: Result<Vec<User>, _> = users
            .iter()
            .map(|user| serde_json::from_value(user.clone()))
            .collect();
        user_vec.map_err(|e| e.into())
    } else {
        Ok(Vec::new())
    }
}

// fn select_user() {
//     get_all_users()
// }
