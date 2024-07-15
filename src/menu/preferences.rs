use crate::utils::common;
use crate::{
    menu::{get_all_profiles, preferences},
    utils::common::highlight_menu_selected,
};
use std::{thread, time};

use crate::menu::profile::Profile;

//todo: receber uma lista do tipo Vec<String>, converter para um array slice e usar
//como options_list
pub fn delete_profile_menu() {
    let selected = 0;
    let option_ = delete_profile_list().unwrap();

    loop {
        highlight_menu_selected(option_list, selected);
    }

    thread::sleep(time::Duration::from_secs(2));
}

fn delete_profile_list() -> Result<(Vec<String>), Box<dyn std::error::Error>> {
    let profile_list = get_all_profiles()?;

    let profile_list_string: Vec<String> = profile_list
        .iter()
        .map(|profile| profile_list.get_profile_name().to_string())
        .collect();

    Ok(profile_list_string)
}

pub fn change_preference_menu() {
    println!("Not implemented yet!");
    thread::sleep(time::Duration::from_secs(2));
}
