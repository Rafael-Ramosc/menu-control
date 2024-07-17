mod menu;
mod utils;

use crossterm::terminal::{self};
use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;

    let mut stdout = stdout();

    utils::common::clear_terminal();

    loop {
        utils::common::clear_terminal();
        menu::main_menu()?;

        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;

    stdout.flush()?;

    Ok(())
}
