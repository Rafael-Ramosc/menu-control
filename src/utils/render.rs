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

use crate::utils::navigate::MenuAction;

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

        // Render options
        for (i, option) in self.options.iter().enumerate() {
            if i == self.selected {
                execute!(stdout, SetForegroundColor(Color::Green))?;
                println!("> {}", option);
                execute!(stdout, ResetColor)?;
            } else {
                println!("  {}", option);
            }
        }

        // Render footer
        let (cols, rows) = size()?;
        execute!(stdout, MoveTo(0, rows - 1), SetForegroundColor(Color::Red))?;
        print!("{:width$}", self.footer, width = cols as usize);
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }

    fn run(&mut self) -> std::io::Result<MenuAction> {
        self.render()?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    self.selected = self.selected.saturating_sub(1);
                    Ok(MenuAction::Navigate(self.selected))
                }
                KeyCode::Down => {
                    self.selected = (self.selected + 1).min(self.options.len() - 1);
                    Ok(MenuAction::Navigate(self.selected))
                }
                KeyCode::Enter => Ok(MenuAction::Select),
                KeyCode::Esc => Ok(MenuAction::Exit),
                _ => Ok(MenuAction::Navigate(self.selected)),
            }
        } else {
            Ok(MenuAction::Navigate(self.selected))
        }
    }
}
