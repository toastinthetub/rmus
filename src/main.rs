// hello seth
use std::{env::args, fs::File, io::BufReader, path::Path};
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let args: Vec<String> = args().collect();
    let filepath = Path::new(&args[1]);

    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(filepath).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(246));
}