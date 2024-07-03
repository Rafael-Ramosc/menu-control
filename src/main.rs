mod menu;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{stdout, Stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode();
    let mut stout = stdout();

    execute!(stout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    let mut selected_option = 1;

    println!("{}", "**** Welcome! ****".green());

    loop {
        println!("Mover cursor: ");
        execute!(stout, cursor::MoveTo(0, 1))?;

        menu::select_menu(selected_option);

        stout.flush()?;
        
        if let Event::Key(ket_event) = event::read()?; {
            match ket_event.code {
                KetCode::Up => select_option.saturating_sub(1).max(1),
                KeyCode::Down => selected_option = (selected_option + 1).min(5),
                KeyCode::Enter => {
                    menu::option_control(selected_option);
                    if selected_option == 5 {
                        break;
                    }
                }
                KeyCode::Esc => break
            }
        }

      
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    Ok(())
}
