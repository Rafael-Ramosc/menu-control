mod menu;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    let mut selected_option = 2;

    println!("{}", "**** Welcome! ****".green());
    thread::sleep(Duration::from_secs(2));

    loop {
        execute!(stdout, cursor::Hide)?;

        menu::select_menu(selected_option);

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            selected_option = match key_event.code {
                KeyCode::Up => selected_option.saturating_sub(1).max(1),
                KeyCode::Down => (selected_option + 1).min(5),
                KeyCode::Enter => {
                    menu::option_control(selected_option)?;
                    if selected_option == 5 {
                        break;
                    }
                    selected_option
                }
                KeyCode::Esc => break,
                _ => selected_option,
            }
        }
        execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;
    }

    terminal::disable_raw_mode()?;
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    Ok(())
}
