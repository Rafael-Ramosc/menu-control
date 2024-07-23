use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::io::{stdout, Write};

pub fn clear_terminal() {
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .expect("Error cleaning terminal");
}

pub fn print_at(x: u16, y: u16, text: &str) -> std::io::Result<()> {
    execute!(stdout(), MoveTo(x, y), Print(text))
}

pub fn print_at_colored(x: u16, y: u16, text: &str, color: Color) -> std::io::Result<()> {
    execute!(
        stdout(),
        MoveTo(x, y),
        SetForegroundColor(color),
        Print(text),
        SetForegroundColor(Color::Reset)
    )
}
