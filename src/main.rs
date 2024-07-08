mod menu;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self},
};
use figlet_rs::FIGfont;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    let mut selected_option: u8 = 1;
    let mut should_continue = true;

    //thread::sleep(Duration::from_secs(2));

    loop {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("Welcome!").unwrap();
        println!("{}", figure.to_string().green());

        let (h, w) = terminal::size().unwrap();
        println!("{}, {}", h, w);

        menu::helpers::select_menu(selected_option);

        println!("{}", "@Rafael Ramos - 2024".bright_magenta());

        stdout.flush()?;

        match menu::key_read_menu(selected_option)? {
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
