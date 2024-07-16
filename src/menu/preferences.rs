use crate::menu;
use crate::menu::profile::Profile;
use crate::utils::common;
use crate::{
    menu::{get_all_profiles, preferences},
    utils::common::{highlight_menu_selected, navigate_menu, MenuAction},
};
use std::{option, thread, time};

pub fn delete_profile_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut selected_option: usize = 0;
    let options = delete_profile_list()?;

    loop {
        common::clear_terminal();

        println!("Delete profile");

        let option_slice: Vec<&str> = options.iter().map(AsRef::as_ref).collect();
        highlight_menu_selected(&option_slice, selected_option);

        match navigate_menu(selected_option, options.len())? {
            MenuAction::Navigate(new_selection) => selected_option = new_selection,
            MenuAction::Select => {
                // Handle profile deletion here
                println!("Deleting profile: {}", options[selected_option]);
                thread::sleep(time::Duration::from_secs(2));
            }
            MenuAction::Back => break,
            MenuAction::Exit => return Ok(()),
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
