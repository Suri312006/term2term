use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::Paths;

#[derive(Deserialize, Debug)]
pub struct Config {
    theme: Theme,
    user: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
}

#[derive(Deserialize, Debug)]
enum Theme {
    Default,
}

impl Config {
    pub fn parse(paths: Paths) -> Config {
        let mut cfg_file = File::open(paths.config_file_path).unwrap();
        let mut buf = String::new();
        cfg_file.read_to_string(&mut buf).unwrap();

        let config: Config = toml::from_str(buf.as_str()).unwrap();

        println!("{:#?}", config);

        config
    }
}
