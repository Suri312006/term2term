use std::{fs::File, io::Read};

use anyhow::{Context, Result};

use serde::Deserialize;

use crate::initialize::gather_paths;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub theme: Theme,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub enum Theme {
    Default,
}

pub fn parse() -> Result<Config> {
    let paths = gather_paths();
    let mut cfg_file = File::open(paths.config_file_path)?;
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;

    let config: Config = toml::from_str(buf.as_str())
        .with_context(|| format!("Unsupported structure for config file."))?;

    Ok(config)
}
