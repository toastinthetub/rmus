// hello seth
mod tui;
mod utils;

use std::{any::Any, env::args, fs::File, path::Path, time::Duration};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleRate, StreamConfig};
use symphonia::core::{audio::{AudioBuffer, AudioBufferRef, Channels, Signal}, codecs::{DecoderOptions, CODEC_TYPE_NULL}, errors::Error, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint};

fn main() {
    let args: Vec<String> = args().collect();

    let filepath = Path::new(args.get(1).unwrap());

    let stdout = tui::initialize_terminal();
    tui::render(stdout);
    std::thread::sleep(Duration::from_secs(2));
    tui::kill_terminal();

    // decoding makes me want to kill myself
    let src = File::open(filepath).unwrap();
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension(filepath.extension().unwrap().to_str().unwrap());

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)
        .unwrap();

    let mut format = probed.format;

    let track = format.tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .unwrap();

    let dec_opts: DecoderOptions = Default::default();

    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)
        .unwrap();

    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.unwrap();

    let mut samples: Vec<f32> = vec![];

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                unimplemented!()
            },
            Err(_) => {
                break;
            }
        };

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder.decode(&packet).unwrap();

        let mut buffer: AudioBuffer<f32> = AudioBuffer::new(decoded.capacity() as u64, *decoded.spec());

        decoded.convert(&mut buffer);

        for &sample in buffer.chan(0) {
            samples.push(sample);
        }
    }

    println!("{}", samples.len());

    // audio shit
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();

    let mut supported_configs_range = device.supported_output_configs().unwrap();
    let supported_config = supported_configs_range.next().unwrap()
        .with_sample_rate(SampleRate {
            0: sample_rate
        });

    let err_fn = |err| eprintln!("{}", err);
    let config: StreamConfig = supported_config.into();
    let channels = config.channels;
    println!("{}", config.sample_rate.0);

    let mut sample_head = 0;

    let stream = device.build_output_stream(&config, move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for frame in data.chunks_mut(channels as usize) {
            let value = samples.get(sample_head).unwrap();
            for sample in frame.iter_mut() {
                *sample = *value;
            }

            sample_head += 1;
        }
    }, err_fn, None).unwrap();

    println!("playing");

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(300))
}
