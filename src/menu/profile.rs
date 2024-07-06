use serde::{Deserialize, Serialize};

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
    user_profile_color: String,
}

impl Configuration {
    pub fn default() -> Configuration {
        Configuration {
            user_profile_color: "RED".to_string(),
        }
    }

    pub fn get_user_profile_color(&self) -> &str {
        &self.user_profile_color
    }
}
