use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, SetForegroundColor},
    terminal, ExecutableCommand,
};
use std::io::stdout;

pub enum MenuAction {
    Navigate(usize),
    Select,
    Back,
    Exit,
}

pub fn navigate_menu(
    current_selection: usize,
    num_options: usize,
) -> Result<MenuAction, Box<dyn std::error::Error>> {
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

pub fn highlight_menu_selected(options: &[&str], selected: usize) {
    let mut stdout = stdout();
    for (i, option) in options.iter().enumerate() {
        if (i as usize + 1) == selected {
            stdout.execute(SetForegroundColor(Color::Green)).unwrap();
            print!("> ");
        } else {
            stdout.execute(SetForegroundColor(Color::White)).unwrap();
            print!("  ");
        }
        println!("{}", option);
    }
    stdout.execute(SetForegroundColor(Color::White)).unwrap();
}
