use serde::{Deserialize, Serialize};

const GREEN_COLOR: u8 = 2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    id: i32,
    profile_name: String,
    pub preferences: Configuration,
}

impl Profile {
    pub fn new(id: i32, profile_name: String) -> Self {
        Self {
            id,
            profile_name,
            preferences: Configuration::default(),
        }
    }

    pub fn get_profile_name(self) -> String {
        self.profile_name
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
