pub mod user;

use crate::menu::user::User;

pub fn intro_menu() {
    println!("Select a option:");
    println!("1. Create new user");
    println!("2. About ME");
    println!("3. EXIT");
}

pub fn select_menu() -> i32 {
    println!("Enter a option number:");
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap();
    option.trim().parse::<i32>().unwrap()
}

pub fn option_control(option: i32) {
    match option {
        1 => {
            println!("Enter your user name: ");
            let mut user_name_select = String::new();
            std::io::stdin().read_line(&mut user_name_select).unwrap();
            let user = User {
                id: 1,
                user_name: user_name_select,
            };
            println!("user created {}", user.user_name);
        }
        2 => println!("My name is Rafael Ramos"),
        3 => println!("Exiting..."),
        _ => println!("Invalid option!"),
    }
}
