mod menu;

fn main() {
    println!("Welcome!");
    loop {
        menu::intro_menu();

        let option = menu::select_menu();

        menu::option_control(option);
    }
}
