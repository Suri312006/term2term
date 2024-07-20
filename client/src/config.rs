use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
    result::Result::Ok,
};

use anyhow::{anyhow, Context, Result};

use serde::{Deserialize, Serialize};

use crate::{
    initialize::gather_paths,
    state::{self, State},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub theme: Theme,
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Theme {
    Default,
}

//TODO: write a default config function that returns  a defualt config? so
//everything is all here in the same place

pub fn parse_config() -> Result<Config> {
    let paths = gather_paths();
    let mut cfg_file = File::open(paths.config_file_path)?;
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;

    let config: Config = toml::from_str(buf.as_str())
        .with_context(|| format!("Unsupported structure for config file."))?;

    Ok(config)
}
