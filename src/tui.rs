use crossterm::{cursor::MoveTo, event::{Event, EventStream, KeyCode, KeyModifiers}, terminal::{self, Clear, ClearType}, QueueableCommand};
use std::{io::{stdout, Stdout, Write}, process, sync::Arc};
use futures::{lock::Mutex, StreamExt};
use crate::{render, AudioState};

const RESOURCE: &str = include_str!("./Resource/resource.txt");

pub async fn event_loop(mut stdout: Stdout, audio_state: Arc<Mutex<AudioState>>) {
    render::render(&mut stdout, audio_state.clone()).await;
    let mut reader = EventStream::new();
    loop {
        let event = reader.next().await.unwrap().unwrap();
        match event {
            Event::Resize(_nw, _nh) => {
                render::render(&mut stdout, audio_state.clone()).await;
            }
            Event::Key(event) => {
                render::render(&mut stdout, audio_state.clone()).await;
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
                // hey sam, ill prob make a better solution for this but for now im gonna constantly render frames so i can print a live status.
                // p.s. i also made render
                // this only renders when an event fires tho, but i need it not to be blocked. maybe u can poll events instead of next()?
                render::render(&mut stdout, audio_state.clone()).await
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
    stdout.queue(MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();
    cook_terminal();

    print!("{}\n", RESOURCE); 
    process::exit(0);
}