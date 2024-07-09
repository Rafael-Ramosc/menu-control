pub mod helpers;
pub mod json;
pub mod profile;

use crate::menu::profile::{Configuration, Profile};
use crate::menu::{self, helpers::*};
use colored::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::style::{Color, SetForegroundColor};
use std::io::{stdout, Write};
use std::process;
use std::{thread, time};

pub fn option_control(option: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    menu::helpers::clear_terminal();

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
            menu::helpers::clear_terminal();
            profiles_list().unwrap();
        }
        3 => {
            menu::helpers::clear_terminal();
            println!("Profiles Preferences:");
            profile_menu();
        }
        4 => about_me().unwrap(),
        5 => {
            menu::helpers::clear_terminal();
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

pub fn key_read_main_menu(
    mut selected_option: u8,
) -> Result<(u8, bool), Box<dyn std::error::Error>> {
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

pub fn select_menu(selected: u8) {
    let options: [&str; 5] = [
        "1. Create new profile",
        "2. List of profiles",
        "3. Profile preferences",
        "4. About",
        "5. EXIT",
    ];

    helpers::highlight_menu_selected(&options, selected);
}

//todo: finish the profile menu
pub fn profile_menu() {
    let options: [&str; 3] = [
        "1. Change preferences",
        "2. Delete profile",
        "3. Back to menu",
    ];

    let mut selected_preference_menu: u8 = 1;

    loop {
        menu::helpers::clear_terminal();
        helpers::highlight_menu_selected(&options, selected_preference_menu);

        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Up => {
                        selected_preference_menu = selected_preference_menu.saturating_sub(1).max(1)
                    }
                    KeyCode::Down => {
                        selected_preference_menu = (selected_preference_menu + 1).min(3)
                    }
                    KeyCode::Enter => match selected_preference_menu {
                        1 => {
                            menu::helpers::clear_terminal();
                            println!("Change preferences");
                            change_preference();
                        }
                        2 => {
                            menu::helpers::clear_terminal();
                            println!("Delete profile");
                            menu::helpers::delete_profile();
                        }
                        3 => {
                            menu::helpers::clear_terminal();
                            break;
                        }
                        _ => {}
                    },
                    KeyCode::Esc => {
                        menu::helpers::clear_terminal();
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}
