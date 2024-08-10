use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, size},
};
use std::{io::{stdout, Write}, sync::mpsc, time::Duration, thread};

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

pub fn new_thread_print (message: String, duration_secs: Duration, location: (u16, u16) ) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    thread::spawn(move || {

        execute!(
            stdout(),
            MoveTo(0, location.1)
        )?;
    
        println!("{}", message);
    
        thread::sleep(duration_secs);

    });

    Ok(())
}
