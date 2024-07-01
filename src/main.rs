mod menu;
use colored::*;

fn main() {
    println!("{}", "**** Welcome! ****".green());
    loop {
        menu::select_menu();

        let option = menu::select_option();

        menu::option_control(option);
    }
}
