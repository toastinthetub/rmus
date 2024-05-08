// hello seth
mod tui;
mod utils;

use std::{env::args, fs::File, io::{self, BufReader}, path::Path, time::Duration};
use rodio::{Decoder, OutputStream, source::Source};
use tui::{initialize_terminal, kill_terminal, render};

fn main() {
    let args: Vec<String> = args().collect();

    let filepath = Path::new(args.get(1).unwrap());

    let mut stdout = tui::initialize_terminal();
    task::spawn(event_loop(stdout));

    // decoding makes me want to kill myself
    let src = File::open(filepath).unwrap();
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension(filepath.extension().unwrap().to_str().unwrap());

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)
        .unwrap();

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