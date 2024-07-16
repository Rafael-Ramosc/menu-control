use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal, ExecutableCommand,
};
use std::io::{stdout, Write};

pub enum MenuAction {
    Navigate(usize),
    Select,
    Back,
    Exit,
}

pub fn navigate_and_highlight_menu(
    options: &[&str],
    current_selection: usize,
) -> Result<MenuAction, Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    let num_options = options.len();

    // Highlight the menu
    for (i, option) in options.iter().enumerate() {
        if i == current_selection {
            execute!(stdout, SetForegroundColor(Color::Green))?;
            print!("> ");
        } else {
            execute!(stdout, SetForegroundColor(Color::White))?;
            print!("  ");
        }
        println!("{}", option);
    }
    execute!(stdout, SetForegroundColor(Color::White))?;
    stdout.flush()?;

    // Handle user input
    if let Event::Key(key_event) = event::read()? {
        if key_event.kind == KeyEventKind::Press {
            return Ok(match key_event.code {
                KeyCode::Up => MenuAction::Navigate(current_selection.saturating_sub(1).max(0)),
                KeyCode::Down => MenuAction::Navigate((current_selection + 1).min(num_options - 1)),
                KeyCode::Enter => MenuAction::Select,
                KeyCode::Tab => MenuAction::Back,
                KeyCode::Esc => MenuAction::Exit,
                _ => MenuAction::Navigate(current_selection),
            });
        }
    }
    Ok(MenuAction::Navigate(current_selection))
}

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
