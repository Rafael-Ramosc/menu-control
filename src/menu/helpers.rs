use crate::menu::{json, profile::Profile};
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use serde_json::Value;
use std::io::{stdout, Write};

//TODO()!: isso aqui deve ser um metodo associado
pub fn create_profile(prompt: &str) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

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
                            let profile: Profile =
                                Profile::new(0, profile_name_select.trim().to_string());
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

pub fn select_menu(selected: u8) {
    let options: [&str; 5] = [
        "1. Create new profile",
        "2. List of profiles",
        "3. Profile preferences",
        "4. About ME",
        "5. EXIT",
    ];

    highlight_menu_selected(&options, selected);
}

pub fn profile_menu(selected: u8) {
    let options: [&str; 5] = [
        "1. Create new profile",
        "2. List of profiles",
        "3. Profile preferences",
        "4. About ME",
        "5. EXIT",
    ];

    highlight_menu_selected(&options, selected);
}

fn highlight_menu_selected(options: &[&str], selected: u8) {
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
