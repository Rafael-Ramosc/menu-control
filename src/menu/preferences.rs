use crate::menu::get_all_profiles;
use crate::utils::common::clear_terminal;
use crate::utils::render::MenuAction;
use crate::utils::render;
use std::{thread, time};

//todo: enumerate list of options??? .enumerate???
pub fn delete_profile_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut profiles = get_all_profiles()?;
    let options: Vec<String> = profiles.iter().map(|p| p.get_profile_name().to_string()).collect();
    

    let mut delete_profile_menu = render::Menu::new(options, 0, "Delete profile".to_string(), "Rafael Ramos - 2024".to_string());

    loop {
        clear_terminal();

        match render::Menu::navigate_control(&mut delete_profile_menu)? {
            MenuAction::Navigate(_) => continue,
            MenuAction::Select => {
                let selected = delete_profile_menu.selected;
                if selected < profiles.len() {
                    let current_status = profiles[selected].get_profile_status();
                    profiles[selected].set_profile_status(!current_status)?;
                    //todo: trocar por um thread novo
                    thread::sleep(time::Duration::from_millis(33));
                } else {
                    println!("Invalid selection!");
                }
            }
            MenuAction::Back => return Ok(()),
            MenuAction::Exit => return Ok(()),
        }
    }

}

pub fn change_preference_menu() {
    println!("Not implemented yet!");
    thread::sleep(time::Duration::from_secs(2));
}
