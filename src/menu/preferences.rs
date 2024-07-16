use crate::menu;
use crate::menu::profile::Profile;
use crate::utils::common::{self, clear_terminal};
use crate::{
    menu::{get_all_profiles, preferences},
    utils::common::{navigate_and_highlight_menu, MenuAction},
};
use std::{option, thread, time};

pub fn delete_profile_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut selected_option = 0;
    let options = delete_profile_list()?;

    loop {
        clear_terminal();
        println!("Delete profile");

        let option_slice: Vec<&str> = options.iter().map(AsRef::as_ref).collect();

        match common::navigate_and_highlight_menu(&option_slice, selected_option)? {
            common::MenuAction::Navigate(new_selection) => selected_option = new_selection,
            common::MenuAction::Select => {
                println!("Deleting profile: {}", options[selected_option]);
                // TODO: Implement actual profile deletion logic here
                thread::sleep(time::Duration::from_secs(2));
            }
            common::MenuAction::Back => break,
            common::MenuAction::Exit => return Ok(()),
        }
    }

    Ok(())
}

fn delete_profile_list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let profile_list = get_all_profiles()?;

    let profile_list_string: Vec<String> = profile_list
        .iter()
        .map(|profile| profile.get_profile_name().to_string())
        .collect();

    Ok(profile_list_string)
}

pub fn change_preference_menu() {
    println!("Not implemented yet!");
    thread::sleep(time::Duration::from_secs(2));
}
