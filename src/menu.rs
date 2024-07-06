pub mod helpers;
pub mod json;
pub mod profile;

use crate::menu::helpers::*;
use crate::menu::profile::{Configuration, Profile};
use colored::*;
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

pub fn select_menu(selected: i32) {
    let mut stdout = stdout();
    let options = [
        "1. Create new profile",
        "2. List of profiles",
        "3. profile preferences",
        "4. About ME",
        "5. EXIT",
    ];

    for (i, option) in options.iter().enumerate() {
        if (i as i32 + 1) == selected {
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

// pub fn select_option() -> i32 {
//     println!("Enter a option number:");
//     let mut option = String::new();
//     std::io::stdin().read_line(&mut option).unwrap();

//     let option = option.trim();

//     match option.parse::<i32>() {
//         Ok(num) => num,
//         Err(_) => 0,
//     }
// }

pub fn option_control(option: i32) -> Result<(), Box<dyn std::error::Error>> {
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
        3 => println!("profiles Preferences:"),
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
