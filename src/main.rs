// hello seth
use std::{env::args, fs::File, io::BufReader};
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    use rodio::{Decoder, OutputStream, source::Source};

// Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("/home/fizbin/lair/snatch/music/Pantera/cowboysfromhell/cowboysfromhell.m4a").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(246));
}