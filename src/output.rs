use crate::error::*;
use crossterm::{cursor, execute, terminal};
use std::io::{self, Write};
use std::{sync::mpsc::Receiver, thread, time::Duration};

pub fn cli(rx: Receiver<String>) -> Result<()> {
    std::thread::spawn(move || {
        let rx = rx;
        let mut message = String::new();
        let mut loader = 0;
        loop {
            if let Ok(msg) = rx.try_recv() {
                message = msg;
            }
            clear_line();
            print!("{} Copying {}", next(loader), message);
            let _ = std::io::stdout().flush();
            loader += 1;
            if loader == 4 {
                loader = 0;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    Ok(())
}
fn next(phase: i8) -> char {
    if phase == 0 {
        '|'
    } else if phase == 1 {
        '/'
    } else if phase == 2 {
        '-'
    } else {
        '\\'
    }
}
pub fn clear_line() {
    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0)
    )
    .unwrap();
}
