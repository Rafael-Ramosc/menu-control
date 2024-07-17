use colored::*;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{size, Clear, ClearType},
};
use figlet_rs::FIGfont;
use std::io::{stdout, Write};

struct Menu {
    options: Vec<String>,
    selected: usize,
    title: String,
    footer: String,
}

impl Menu {
    fn new(options: Vec<String>, title: String, footer: String) -> Self {
        Self {
            options,
            selected: 0,
            title,
            footer,
        }
    }

    fn render(&self) -> std::io::Result<()> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Render title
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert(&self.title).unwrap();
        println!("{}", figure.to_string().green());

        // Render footer
        let (cols, rows) = size()?;
        execute!(stdout, MoveTo(0, rows - 1), SetForegroundColor(Color::Red))?;
        print!("{:width$}", self.footer, width = cols as usize);
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }
}
