// hello seth
mod tui;
mod utils;
mod render;

use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use symphonia::core::{audio::{AudioBuffer, Signal}, codecs::{DecoderOptions, CODEC_TYPE_NULL}, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleRate, StreamConfig};
use std::{env::args, fs::File, io::{ErrorKind, Read, Write}, path::Path, sync::Arc, time::Duration};

use tokio::task;

use crate::tui::event_loop;

struct AudioState {
    status: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    library_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            library_path: ".".to_string(),
        }
    }
}

impl Config {
    fn load() -> Self {
        match File::open("config.toml") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                toml::from_str(buf.as_str()).unwrap()
            },
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    // load default
                    let config: Config = Default::default();
                    let mut file = File::create("config.toml").unwrap();
                    let buf = toml::to_string_pretty(&config).unwrap();
                    file.write_all(buf.as_bytes()).unwrap();
                    config
                } else {
                    panic!("{}", err)
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::load();

    let args: Vec<String> = args().collect();

    let filepath = Path::new(args.get(1).unwrap());

    let audio_state = Arc::new(Mutex::new(AudioState {
        status: "".to_string(),
    }));

    let stdout = tui::initialize_terminal();
    task::spawn(event_loop(stdout, audio_state.clone(), config.clone()));

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
    let total_frames = track.codec_params.n_frames.unwrap();

    let mut samples: Vec<f32> = vec![];
    let mut frame_count = 0;

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(symphonia::core::errors::Error::ResetRequired) => {
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

        frame_count += decoded.frames();
        set_status(format!("\rDecoding... {:.2}% ({} / {})", frame_count as f32 / total_frames as f32 * 100.0, frame_count, total_frames), audio_state.clone()).await;
    }

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

    set_status("Playing".to_string(), audio_state.clone()).await;

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(300))
}

async fn set_status(status: String, audio_state: Arc<Mutex<AudioState>>) {
    let mut state = audio_state.lock().await;
    state.status = status;
}
