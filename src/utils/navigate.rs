use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, SetForegroundColor},
};
use std::io::{stdout, Write};

pub enum MenuAction {
    Navigate(usize),
    Select,
    Back,
    Exit,
}

pub fn navigate_control(
    options: &[&str],
    current_selection: usize,
) -> Result<MenuAction, Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    let num_options = options.len();

    // Highlight the menu
    //todo: create a enum to choose the color and select character
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
    //todo: i need to create a way to the key pree come from a enum to futher customization
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
