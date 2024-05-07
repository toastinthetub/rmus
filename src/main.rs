// hello seth
mod tui;
mod utils;

use std::{env::args, fs::File, io::{self, BufReader}, path::Path, time::Duration};
use rodio::{Decoder, OutputStream, source::Source};
use tui::{initialize_terminal, kill_terminal, render};

fn main() {
    let args: Vec<String> = args().collect();

    assert!(args.len() >= 2, "no file provided");

    let filepath = Path::new(&args[1]);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = BufReader::new(File::open(filepath).unwrap());
    let stdout = tui::initialize_terminal();
    tui::render(stdout);
    std::thread::sleep(Duration::from_secs(2));
    tui::kill_terminal();

    let source = Decoder::new(file).unwrap();
    let duration = source.total_duration().unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    std::thread::sleep(duration);
}