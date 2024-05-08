// hello seth
mod tui;
mod utils;
mod render;

use std::{env::args, fs::File, io::{self, BufReader, Write}, path::Path, thread, time::Duration};
use crossterm::{cursor::MoveTo, QueueableCommand};
use rodio::{Decoder, OutputStream, source::Source};
use tui::{initialize_terminal, kill_terminal};
use render::render;

use crate::tui::event_loop;


fn main() {
    let args: Vec<String> = args().collect();

    assert!(args.len() >= 2, "no file provided");

    let filepath = Path::new(&args[1]);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = BufReader::new(File::open(filepath).unwrap());
    let mut stdout = initialize_terminal();


    let handle = thread::spawn(move || {
        stdout.queue(MoveTo(0, 0));
        stdout.write("FUCKING SKILL ISSUE".as_bytes());
        event_loop(stdout)
    });

    handle.join().unwrap();

    let source = Decoder::new(file).unwrap();
    let duration = source.total_duration().unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(duration);
}