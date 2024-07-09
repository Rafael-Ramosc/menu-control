mod menu;
mod utils;

use colored::*;
use crossterm::cursor;
use crossterm::terminal::{self};
use figlet_rs::FIGfont;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use utils::render::calculate_column;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    menu::helpers::clear_terminal();
    let mut selected_option: u8 = 1;

    //thread::sleep(Duration::from_secs(2));

    loop {
        menu::helpers::clear_terminal();

        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("Menu Control!").unwrap();
        println!("{}", figure.to_string().green());

        let (h, w) = terminal::size().unwrap();

        menu::select_menu(selected_option);

        //reminder(): i need to calculate the size of the columns
        cursor::MoveTo(200, 20);

        calculate_column(0, 20);

        println!(
            "                                         {}",
            "@Rafael Ramos - 2024".red()
        );
        println!("Terminal Size: {}, {}", h, w);

        stdout.flush()?;

        //todo: fix infinite loop when trying to exit (when press ESC)
        match menu::key_read_main_menu(selected_option)? {
            (new_option, true) => {
                selected_option = new_option;
            }
            (_, false) => {
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;

    stdout.flush()?;

    Ok(())
}
