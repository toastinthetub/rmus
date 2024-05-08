use crossterm::{cursor::MoveTo, event::{self, poll, read, Event, EventStream, KeyCode, KeyModifiers}, execute, terminal::{self, Clear, ClearType}, QueueableCommand};
use std::{io::{stdout, Stdout, Write}, process, time::Duration, thread, env};
use futures::StreamExt;
use crate::render;

const resource: &str = include_str!("./resource.txt");

pub async fn event_loop(mut stdout: Stdout) {
    render(&stdout);
    let mut reader = EventStream::new();
    loop {
        let event = reader.next().await.unwrap().unwrap();
        match event {
            Event::Resize(nw, nh) => {
                render::render(&mut stdout)
            }
            Event::Key(event) => {
                render(&mut stdout);
                match event.code {
                    KeyCode::Char(x) => {
                        if x == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                            kill_terminal();
                        } else {}
                    }
                    _ => {
                        
                    }
                }
            }
            _ => {
                
            }
        }
    }
}

pub fn initialize_terminal() -> Stdout {
    let mut stdout = stdout();
    let _ = terminal::enable_raw_mode();

    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.flush();

    stdout
}

pub fn cook_terminal() {
    let _ = terminal::disable_raw_mode();
}

pub fn kill_terminal() {
    let mut stdout = stdout();

    stdout.queue(Clear(ClearType::All)).unwrap();
    cook_terminal();

    print!("{}", resource) 
}