use crate::menu::json::update_profile_status_in_json;
use crate::menu::get_all_profiles;
use crate::utils::common::clear_terminal;
use crate::utils::navigate::{navigate_control, MenuAction};
use std::{thread, time};

//todo: need to implement the menu here
pub fn delete_profile_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut selected_option = 0;
    // let options = delete_profile_list()?;

    loop {
        clear_terminal();
        println!("Delete profile");

       let mut profiles = get_all_profiles()?;

        let options: Vec<String> = profiles.iter().map(|p| p.get_profile_name().to_string()).collect();
        let option_slice: Vec<&str> = options.iter().map(AsRef::as_ref).collect();

        match navigate_control(&option_slice, selected_option)? {
            MenuAction::Navigate(new_selection) => selected_option = new_selection,
            MenuAction::Select => {
                match profiles[selected_option].get_profile_status() {
                  true =>  profiles[selected_option].set_profile_status(false),
                  false=>  profiles[selected_option].set_profile_status(true),
                }
               
                thread::sleep(time::Duration::from_secs(1));
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
