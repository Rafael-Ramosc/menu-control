use std::io::stdout;

use crossterm::{
    cursor, execute,
    style::{Color, SetForegroundColor},
    terminal, ExecutableCommand,
};

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

pub fn highlight_menu_selected(options: &[&str], selected: u8) {
    let mut stdout = stdout();
    for (i, option) in options.iter().enumerate() {
        if (i as u8 + 1) == selected {
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
