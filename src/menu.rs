pub mod helpers;
pub mod json;
pub mod profile;

use crate::menu::helpers::*;
use crate::menu::profile::{Configuration, Profile};
use colored::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    cursor,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use crossterm::{execute, terminal};
use std::io::{stdout, Write};
use std::process;
use std::{thread, time};

pub fn option_control(option: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    match option {
        1 => match create_profile("Enter your profile name (or press TAB to return to menu):") {
            Ok(Some(profile)) => {
                let profile_name = Profile::get_profile_name(profile);
                println!("{}", " SUCESS".green());
                println!(" profile {} Created!", profile_name.red());
                thread::sleep(time::Duration::from_secs(2));
            }
            Ok(None) => println!("profile creation cancelled."),
            Err(e) => println!("Error creating profile: {}", e),
        },
        2 => {
            execute!(
                stdout,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            profiles_list().unwrap();
        }
        3 => {
            execute!(
                stdout,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            println!("Profiles Preferences:");
            profile_menu(option);
        }
        4 => about_me().unwrap(),
        5 => {
            execute!(
                stdout,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            println!("Closing...");
            process::exit(0);
        }
        _ => println!("Invalid option!"),
    }

    println!("\nPress any key to return to the menu...");
    stdout.flush()?;
    crossterm::event::read()?;

    Ok(())
}

pub fn key_read_menu(mut selected_option: u8) -> Result<(u8, bool), Box<dyn std::error::Error>> {
    if let Event::Key(key_event) = event::read()? {
        if key_event.kind == KeyEventKind::Press {
            selected_option = match key_event.code {
                KeyCode::Up => selected_option.saturating_sub(1).max(1),
                KeyCode::Down => (selected_option + 1).min(5),
                KeyCode::Enter => {
                    option_control(selected_option)?;
                    if selected_option == 5 {
                        return Ok((selected_option, false));
                    }
                    selected_option
                }
                KeyCode::Esc => return Ok((selected_option, false)),
                _ => selected_option,
            };
        }
    }
    Ok((selected_option, true))
}
