use crate::error::*;
use crossterm::{cursor, execute, terminal};
use std::io;
use std::{sync::mpsc::Receiver, thread, time::Duration};
struct Message(String);

pub fn cli(rx: Receiver<String>) -> Result<()> {
    std::thread::spawn(move || {
        let rx = rx;
        let mut message = Message(String::new());
        let mut loader = 0;
        loop {
            if let Ok(msg) = rx.try_recv() {
                message.0 = msg;
            }
            clear_line();
            print!("{} Copying {}", next(loader), message.0);
            loader += 1;
            if loader == 4 {
                loader = 0;
            }
            thread::sleep(Duration::from_secs(1));
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
fn clear_line() {
    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0)
    )
    .unwrap();
}
impl Drop for Message {
    fn drop(&mut self) {
        clear_line();
        println!("Backup finished");
    }
}
