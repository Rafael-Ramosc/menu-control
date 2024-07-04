use crate::menu::{json, user::User};
use colored::*;
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use serde_json::Value;
use std::io::{stdout, Write};

pub fn create_user() -> Result<User, Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    println!(" ------- Creating new user -------");
    println!("Enter your user name: ");
    stdout.flush()?;

    let mut user_name_select = String::new();
    std::io::stdin().read_line(&mut user_name_select)?;
    let user_name_select = user_name_select.trim().to_string();

    let user: User = User::new(0, user_name_select);
    let new_user_json = serde_json::to_string_pretty(&user)?;

    json::json_data(&new_user_json)?;
    println!("{:?}", new_user_json);

    Ok(user)
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
        Err(_e) => {
            // println!("Error when trying to read users: {}", e);
            //Will return 0 because there is no list
            0
        }
    }
}
