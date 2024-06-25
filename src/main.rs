mod menu;

fn main() {
    println!("**** Welcome! ****");
    loop {
        menu::select_menu();

        let option = menu::select_option();

        menu::option_control(option);
    }
}
