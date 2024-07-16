mod menu;
mod utils;

use colored::*;
use crossterm::cursor;
use crossterm::terminal::{self};
use std::io::{stdout, Write};
use utils::render::calculate_column;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;

    let mut stdout = stdout();

    utils::common::clear_terminal();

    loop {
        utils::common::clear_terminal();
        menu::main_menu()?;

        calculate_column(0, 20);

        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;

    stdout.flush()?;

    Ok(())
}
