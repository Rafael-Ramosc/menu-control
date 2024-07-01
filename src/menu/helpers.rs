use crate::menu::{json, user::User};
use colored::*;
use serde_json::Value;

pub fn create_user() -> User {
    println!(" ------- Creating new user -------");

    println!("Enter your user name: ");
    let mut user_name_select = String::new();
    std::io::stdin().read_line(&mut user_name_select).unwrap();
    let user_name_select = user_name_select.trim().to_string();

    let id_index = get_user_lenght();

    let user: User = User::new(id_index, user_name_select);
    let new_user_json =
        serde_json::to_string_pretty(&user).expect("Error converting String to Json");

    json::json_data(&new_user_json).expect("Error when trying to write json");
    println!("{:?}", new_user_json);
    user
}

pub fn users_list() {
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

pub fn get_user_lenght() -> i32 {
    match get_all_users() {
        Ok(list) => list.len().try_into().unwrap(),
        Err(e) => {
            println!("Error when trying to read users: {}", e);
            0
        }
    }
}
