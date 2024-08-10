pub mod helpers;
pub mod json;
pub mod preferences;
pub mod profile;

use crate::menu::profile::{Configuration, Profile};
use crate::menu::{self, helpers::*};
use crate::utils::common::{clear_terminal, print_at_colored};
use crate::utils::render::MenuAction;
use crate::utils::{self, render};
use colored::*;
use crossterm::event::{self};
use std::process;
use std::{thread, time};

// const CREATER_FOOTER: String = "Rafael Ramos - 2024".to_string();

pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    let options: Vec<String> = vec![
    "1. Create new profile".to_string(),
    "2. List of profiles".to_string(),
    "3. Profile preferences".to_string(),
    "4. About".to_string(),
    "5. Exit".to_string(),
];

    let mut main_menu = render::Menu::new(options, 0, "Menu Control".to_string(),  "Rafael Ramos - 2024".to_string());

    loop {
        clear_terminal();

        match render::Menu::navigate_control(&mut main_menu)? {
            MenuAction::Navigate(_) => continue,
            MenuAction::Select => {
                match main_menu.selected {
                    0 => match create_profile(
                        "Enter your profile name (or press TAB to return to menu):",
                    ) {
                        Ok(Some(profile)) => {
                            let profile_name = Profile::get_profile_name(&profile);
                            println!("{}", " SUCCESS".green());
                            println!(" profile {} Created!", profile_name.red());
                            thread::sleep(time::Duration::from_secs(2));
                        }
                        Ok(None) => println!("profile creation cancelled."),
                        Err(e) => println!("Error creating profile: {}", e),
                    },
                    1 => {
                        utils::common::clear_terminal();
                        profiles_list().unwrap();
                    }
                    2 => {
                        utils::common::clear_terminal();
                        println!("Profiles Preferences:");
                        profile_menu()?;
                    }
                    3 => about_me().unwrap(),
                    4 => {
                        utils::common::clear_terminal();
                        println!("Closing...");
                        process::exit(0);
                    }
                    _ => println!("Invalid option!"),
                }
                println!("\nPress any key to return to the menu...");
                event::read()?;
            }
            MenuAction::Back => return Ok(()),
            MenuAction::Exit => {
                utils::common::clear_terminal();
                println!("Closing...");
                process::exit(0)
            },
        }
    }
}

//todo: finish the profile menu
pub fn profile_menu() -> Result<(), Box<dyn std::error::Error>> {
    let options: Vec<String> = vec![
        "1. Change preferences".to_string(),
        "2. Delete profile".to_string(),
        "3. Back to menu".to_string(),
    ];
   

   let mut profile_menu = render::Menu::new(options, 0, "Profile Menu".to_string(), "Rafael Ramos - 2024".to_string());

    loop {
        clear_terminal();

        match render::Menu::navigate_control( &mut profile_menu)?{
            MenuAction::Navigate(_) => continue,
            MenuAction::Select => {
                match profile_menu.selected {
                    0 => {
                        clear_terminal();
                        println!("Change preferences");
                        menu::preferences::change_preference_menu();
                    }
                    1 => {
                        clear_terminal();
                        match preferences::delete_profile_menu() {
                            Ok(_) => println!("Profile deleted successfully."),
                            Err(e) => println!("An error occurred while deleting profile: {}", e),
                        }
                    }
                    2 => return Ok(()),
                    _ => println!("Invalid option!"),
                }
                println!("\nPress any key to continue...");
                event::read()?;
            }
            MenuAction::Back =>  return Ok(()),
            MenuAction::Exit => return Ok(()),

        }
    }

}
