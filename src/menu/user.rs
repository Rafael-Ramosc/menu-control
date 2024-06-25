pub struct User {
    id: i32,
    user_name: String,
    // pub dat_create: Date,
    // pub preferences: Preferences,
}

struct Preferences {
    font_color: String,
    font_size: i32,
}

impl User {
    pub fn new(id: i32, user_name: String) -> Self {
        Self { id, user_name }
    }

    pub fn default() -> User {
        User {
            id: 0,
            user_name: String::from("default_profile"),
        }
    }

    pub fn get_username(self) -> String {
        self.user_name
    }
}
