use std::io::stdout;

use crossterm::{cursor, execute, terminal};

//todo(): puts here all function that i use in almost every file: ->clean_terminal()
pub fn clear_terminal() {
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .expect("Error cleaning terminal");
}
