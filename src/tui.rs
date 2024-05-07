use crossterm::{cursor::MoveTo, event::{self, poll, read, Event, EventStream, KeyCode, KeyModifiers}, terminal::{self, Clear, ClearType}, QueueableCommand};
use std::{io::{stdout, Stdout, Write}, process};

pub fn render(mut stdout: Stdout) {
    let (w, h) = terminal::size().unwrap();

    let bar_char = "🮙";
    let bar = bar_char.repeat(w as usize);

    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.queue(MoveTo(0, 1));
    stdout.flush().unwrap();
    stdout.write(bar.as_bytes()).unwrap();
    let _ = stdout.queue(MoveTo(w, h - 1));
    stdout.write(bar_char.repeat(w as usize).as_bytes()).unwrap();
    stdout.flush().unwrap();
    let _ = stdout.queue(MoveTo(w / 2, h / 2));
    stdout.flush().unwrap();
}

pub fn initialize_terminal() -> Stdout {
    let mut stdout = stdout();
    let _ = terminal::enable_raw_mode();

    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.flush();

    stdout
}

pub fn kill_terminal() {
    let _ = terminal::disable_raw_mode();
}