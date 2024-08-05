use serde::{Deserialize, Serialize};

const GREEN_COLOR: u8 = 2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    id: i32,
    profile_name: String,
    is_blocked: bool,
    pub preferences: Configuration,
}

impl Profile {
    pub fn new(id: i32, profile_name: String, is_blocked: bool) -> Self {
        Self {
            id,
            profile_name,
            is_blocked,
            preferences: Configuration::default(),
        }
    }

    pub fn get_profile_name(&self) -> &String {
        &self.profile_name
    }

    pub fn get_profile_id(&self) -> i32 {
        self.id
    }

    pub fn get_profile_status(&self) -> bool {
        self.is_blocked
    }

    pub fn set_profile_status(&mut self, status: bool) {
        println!("Profile status: {}", status);
        self.is_blocked = status;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    user_profile_color: u8,
}

impl Configuration {
    pub fn default() -> Configuration {
        Configuration {
            user_profile_color: GREEN_COLOR,
        }
    }

    pub fn get_profile_color(&self) -> &str {
        let color = match &self.user_profile_color {
            1 => "RED",
            2 => "GREEN",
            3 => "YELLOW",
            4 => "BLUE",
            5 => "MAGENTA",
            6 => "CYAN",
            7 => "WHITE",
            _ => "RED",
        };
        color
    }

    pub fn set_profile_color(&mut self, color_selected: u8) {
        self.user_profile_color = color_selected;
    }
}
