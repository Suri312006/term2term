
use std::{fs::File, io::Read};

use anyhow::{Context, Result};

use keyring::Entry;
use serde::Deserialize;

use crate::Paths;

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

pub fn parse(paths: Paths) -> Result<Config> {
    let mut cfg_file = File::open(paths.config_file_path)?;
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;

    let mut config: Config = toml::from_str(buf.as_str())
        .with_context(|| format!("Unsupported structure for config file."))?;

    let entry = Entry::new("term2term", config.user.name.as_str()).unwrap();

    config.user.id = entry.get_password()?;

    Ok(config)
}
