// hello seth
mod tui;
mod utils;

use std::{env::args, ffi::OsStr, fs::File, io::{self, BufReader}, path::Path, slice::Iter, sync::Arc, time::Duration};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, Sample, StreamConfig};
use symphonia::core::{audio::SampleBuffer, codecs::{DecoderOptions, CODEC_TYPE_NULL}, errors::Error, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint};
use tui::{initialize_terminal, kill_terminal, render};

fn main() {
    let args: Vec<String> = args().collect();

    assert!(args.len() >= 2, "no file provided");

    let filepath = Path::new(&args[1]);

    let file = File::open(filepath).unwrap();
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension(filepath.extension().unwrap_or(OsStr::new("")).to_str().unwrap());

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .unwrap();

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .unwrap();

    let dec_opts: DecoderOptions = Default::default();

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .unwrap();

    let track_id = track.id;

    let mut sample_count = 0;
    let mut sample_buf = None;

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                unimplemented!();
            }
            Err(err) => {
                break;
            }
        };

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                if sample_buf.is_none() {
                    let spec = *decoded.spec();

                    let duration = decoded.capacity() as u64;

                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }

                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(decoded);

                    sample_count += buf.samples().len();
                    print!("\rDecoded {} samples", sample_count);
                }
            },
            Err(Error::IoError(_)) => {
                continue;
            },
            Err(Error::DecodeError(_)) => {
                continue;
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    let mut samples_iter = sample_buf.unwrap().samples().iter();

    let stdout = tui::initialize_terminal();
    tui::render(stdout);
    std::thread::sleep(Duration::from_secs(2));
    tui::kill_terminal();

    // audio shit
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();

    let mut supported_configs_range = device.supported_output_configs().unwrap();
    let supported_config = supported_configs_range.next().unwrap()
        .with_max_sample_rate();

    let err_fn = |err| eprintln!("{}", err);
    let config: StreamConfig = supported_config.into();
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut next_value = move || {
        *samples_iter.next().unwrap() as f32
    };

    let stream = device.build_output_stream(&config, move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
        write_data(data, channels, &mut next_value)
    }, err_fn, None).unwrap();

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(5))
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
