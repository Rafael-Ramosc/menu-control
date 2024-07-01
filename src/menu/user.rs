use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    user_name: String,
    pub preferences: Configuration,
}

impl User {
    pub fn new(id: i32, user_name: String) -> Self {
        Self {
            id,
            user_name,
            preferences: Configuration::default(),
        }
    }

    pub fn get_user() {}

    pub fn get_username(self) -> String {
        self.user_name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    user_name_color: String,
}

impl Configuration {
    pub fn default() -> Configuration {
        Configuration {
            user_name_color: "RED".to_string(),
        }
    }

    pub fn get_user_name_color(&self) -> &str {
        &self.user_name_color
    }
}
