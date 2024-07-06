use crate::menu::{json, user::User};
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use serde_json::Value;
use std::io::{stdout, Write};

//this function will follow the Separation of Concerns principle
pub fn create_user(prompt: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    println!(" ------- Creating new user -------");
    println!("{}", prompt);
    stdout.flush()?;

    let mut user_name_select = String::new();

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char(c) => {
                        user_name_select.push(c);
                        print!("{}", c);
                        stdout.flush()?;
                    }
                    KeyCode::Backspace => {
                        if !user_name_select.is_empty() {
                            user_name_select.pop();
                            print!("\x08 \x08");
                            stdout.flush()?;
                        }
                    }
                    KeyCode::Enter => {
                        if !user_name_select.is_empty() {
                            let user: User = User::new(0, user_name_select.trim().to_string());
                            let new_user_json = serde_json::to_string_pretty(&user)?;
                            json::json_data(&new_user_json)?;
                            return Ok(Some(user));
                        }
                    }
                    KeyCode::Tab => {
                        println!("\nReturning to menu...");
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn users_list() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "User list:".yellow());
    match get_all_users() {
        Ok(users) => {
            let mut count = 0;
            for user in users {
                println!("{} User: {}", count, user.get_username());
                count += 1;
            }
            println!("TOTAL OF USERS: {}", count.to_string().yellow());
        }
        Err(e) => {
            println!("Error when trying to read users: {}", e);
        }
    }

    println!("Press Tab to return to menu...");

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Tab => {
                        println!("Returning to menu...");
                        return Ok(());
                    }
                    _ => {}
                }
            }
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

// pub fn get_user_lenght() -> i32 {
//     match get_all_users() {
//         Ok(list) => list.len().try_into().unwrap(),
//         Err(_e) => {
//             // println!("Error when trying to read users: {}", e);
//             //Will return 0 because there is no list
//             0
//         }
//     }
// }

pub fn about_me() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    println!(
        "{}",
        "Author: Rafael Ramos email:rafael.ramosrc@gmail.com".green()
    );

    println!("Press Tab to return to menu...");

    stdout.flush()?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Tab => {
                        println!("Returning to menu...");
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}
