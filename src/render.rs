use crossterm::{cursor::MoveTo, terminal::{self, Clear, ClearType}, QueueableCommand};
use futures::lock::Mutex;
use std::{io::{Stdout, Write}, sync::Arc};

use crate::{utils::get_artists_albums_songs, AudioState};

pub async fn render(stdout: &mut Stdout, audio_state: Arc<Mutex<AudioState>>) {
    let (w, h) = terminal::size().unwrap();
    let binder = "█".repeat(w as usize);
    let _bar = binder.as_bytes();

    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.flush().unwrap();


    horizontal_bar(stdout, "█".to_string(), w - w, h - h, w);
    horizontal_bar(stdout, "█".to_string(), w - w, h, w);
    vertical_bar(stdout, "██".to_string(), (w / 3) + (w / 3) , 0, h);
    vertical_bar(stdout, "█".to_string(), w - w , 0, h);
    vertical_bar(stdout, "█".to_string(), w , 0, h);

    let music_folder_path = std::path::Path::new("/home/mct32/Music"); 
    write_artists(stdout, || {
        get_artists_albums_songs(&music_folder_path).unwrap().0
    }, w, h);

    let state = audio_state.lock().await;
    stdout.queue(MoveTo(0, h)).unwrap();
    stdout.write_all(state.status.as_bytes()).unwrap();
    drop(state);
    
    stdout.queue(MoveTo(w / 2, h / 2)).unwrap();
    stdout.flush().unwrap();
}

pub fn write_artists<F>(stdout: &mut Stdout, artist_generator: F, w: u16, _h: u16)
where
    F: Fn() -> Vec<String>,
{
    let artists = artist_generator();
    for (index, artist) in artists.iter().enumerate() {
        stdout.queue(MoveTo(w - w + 1, (index + 1) as u16)).unwrap();
        stdout.write_all(artist.as_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
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

pub fn horizontal_bar(mut stdout: &Stdout, char: String, x: u16, y: u16, length: u16) {
    stdout.queue(MoveTo(x, y)).unwrap();
    stdout.flush().unwrap();
    for cell in 0..=length {
        stdout.write(char.as_bytes()).unwrap();
        stdout.queue(MoveTo(cell, y)).unwrap();
        stdout.flush().unwrap();
    }
}