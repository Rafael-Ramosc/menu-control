mod menu;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    terminal::{self, ClearType},
    QueueableCommand,
};
use figlet_rs::FIGfont;
use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    let mut selected_option = 1;

    //thread::sleep(Duration::from_secs(2));

    loop {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("Welcome!").unwrap();
        println!("{}", figure.to_string().green());

        let (mut h, mut w) = terminal::size().unwrap();
        println!("{}, {}", h, w);

        menu::select_menu(selected_option);

        println!("{}", "@Rafael Ramos - 2024".bright_magenta());

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
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
                };
            }
        }
    }

    terminal::disable_raw_mode()?;

    stdout.flush()?;

    Ok(())
}
