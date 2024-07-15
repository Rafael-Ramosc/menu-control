use crate::menu::{self, json, profile::Profile};
use crate::utils;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, SetForegroundColor},
    terminal::{self},
    ExecutableCommand,
};
use serde_json::Value;
use std::io::{stdout, Write};

pub fn create_profile(prompt: &str) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    utils::common::clear_terminal();

    println!(" ------- Creating new profile -------");
    println!("{}", prompt);
    stdout.flush()?;

    let mut profile_name_select = String::new();

    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char(c) => {
                        profile_name_select.push(c);
                        print!("{}", c);
                        stdout.flush()?;
                    }
                    KeyCode::Backspace => {
                        if !profile_name_select.is_empty() {
                            profile_name_select.pop();
                            print!("\x08 \x08");
                            stdout.flush()?;
                        }
                    }
                    KeyCode::Enter => {
                        if !profile_name_select.is_empty() {
                            //get the next id
                            let mut profile_next_id: i32 = 0;
                            match get_all_profiles() {
                                Ok(profiles) => profile_next_id = profiles.len() as i32,
                                Err(_) => profile_next_id = 0,
                            }

                            let profile: Profile = Profile::new(
                                profile_next_id,
                                profile_name_select.trim().to_string(),
                                false,
                            );
                            let new_profile_json = serde_json::to_string_pretty(&profile)?;
                            json::json_data(&new_profile_json)?;
                            return Ok(Some(profile));
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

pub fn profiles_list() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "profile list:".yellow());
    match get_all_profiles() {
        Ok(profiles) => {
            let mut count = 0;
            for profile in profiles {
                println!("{} profile: {}", count, profile.get_profile_name());
                count += 1;
            }
            println!("TOTAL OF PROFILES: {}", count.to_string().yellow());
        }
        Err(e) => {
            println!("Error when trying to read profiles: {}", e);
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

pub fn get_all_profiles() -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string("data/profile.json")?;
    let json: Value = serde_json::from_str(&file_content)?;

    if let Some(profiles) = json["profiles"].as_array() {
        let profile_vec: Result<Vec<Profile>, _> = profiles
            .iter()
            .map(|profile| serde_json::from_value(profile.clone()))
            .collect();
        profile_vec.map_err(|e| e.into())
    } else {
        Ok(Vec::new())
    }
}

pub fn about_me() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    utils::common::clear_terminal();

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

pub fn highlight_menu_selected(options: &[&str], selected: u8) {
    let mut stdout = stdout();
    for (i, option) in options.iter().enumerate() {
        if (i as u8 + 1) == selected {
            stdout.execute(SetForegroundColor(Color::Green)).unwrap();
            print!("> ");
        } else {
            stdout.execute(SetForegroundColor(Color::White)).unwrap();
            print!("  ");
        }
        println!("{}", option);
    }
    stdout.execute(SetForegroundColor(Color::White)).unwrap();
}
