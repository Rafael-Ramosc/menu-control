use colored::*;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{size, Clear, ClearType},
};
use figlet_rs::FIGfont;
use std::io::{stdout, Write};

#[derive(Debug)]
pub enum MenuAction {
    Navigate(usize),
    Select,
    Back,
    Exit,
}

pub struct Menu {
    options: Vec<String>,
    selected: usize,
    title: String,
    footer: String,
}

impl Menu {
   pub fn new(options: Vec<String>, selected: usize, title: String, footer: String) -> Self {
        Self {
            options,
            selected,
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
                print!("> ");
            } else {
                execute!(stdout, SetForegroundColor(Color::White))?;
                print!("  ");
            }
            println!("{}", option);
        }
        execute!(stdout, SetForegroundColor(Color::White))?;

        // Render footer
        let (cols, rows) = size()?;
        execute!(stdout, MoveTo(0, rows - 1), SetForegroundColor(Color::Red))?;
        print!("{:width$}", self.footer, width = cols as usize);
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }

    pub fn navigate_control(&mut self) -> Result<MenuAction, Box<dyn std::error::Error>> {
        self.render()?;

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                return Ok(match key_event.code {
                    KeyCode::Up => {
                        self.selected = self.selected.saturating_sub(1);
                        MenuAction::Navigate(self.selected)
                    }
                    KeyCode::Down => {
                        self.selected = (self.selected + 1).min(self.options.len() - 1);
                        MenuAction::Navigate(self.selected)
                    }
                    KeyCode::Enter => MenuAction::Select,
                    KeyCode::Tab => MenuAction::Back,
                    KeyCode::Esc => MenuAction::Exit,
                    _ => MenuAction::Navigate(self.selected),
                });
            }
        }
        Ok(MenuAction::Navigate(self.selected))
    }
}