use crossterm::{cursor::MoveTo, event::{self, poll, read, Event, EventStream, KeyCode, KeyModifiers}, execute, terminal::{self, Clear, ClearType}, QueueableCommand};
use std::{io::{stdout, Stdout, Write}, process};

pub fn render(mut stdout: &Stdout) {
    let (w, h) = terminal::size().unwrap();
    let binder = "█".repeat(w as usize);
    let bar = binder.as_bytes();

    stdout.queue(Clear(ClearType::All)).unwrap();

    stdout.flush().unwrap();

    stdout.queue(MoveTo(w - w, h - h)).unwrap();
    stdout.write(bar).unwrap();

    stdout.flush().unwrap();

    stdout.queue(MoveTo(0, h)).unwrap();
    stdout.write(bar).unwrap();

    vertical_bar(&mut stdout, "██".to_string(), (w / 3) + (w / 3) , 0, h);
    stdout.queue(MoveTo(w / 2, h / 2)).unwrap();

    stdout.flush().unwrap();
}


pub fn vertical_bar(mut stdout: &Stdout, char: String,  x: u16, start_y: u16, end_y: u16) {
    let bar_height = end_y - start_y;

    stdout.queue(MoveTo(x, start_y)).unwrap();
    stdout.flush().unwrap();
    for cell in 0..=bar_height {
        stdout.write(char.as_bytes()).unwrap();
        stdout.queue(MoveTo(x, cell)).unwrap();
        stdout.flush().unwrap();
    }
}