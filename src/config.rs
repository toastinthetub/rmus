use std::{fs::File, io::{ErrorKind, Read, Write}};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub library_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            library_path: ".".to_string(),
        }
    }
}

impl Config { 
    pub fn load() -> Self {
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