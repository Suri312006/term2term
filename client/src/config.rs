use std::{fs::File, io::Read};

use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};

use crate::initialize::gather_paths;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub theme: Theme,
    pub user: User,
    pub curr_convo: Option<Conversations>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Conversations {
    // somethign about users
    id: String,
    user1_id: String,
    user2_id: String,
}

//TODO: write a default config function that returns  a defualt config? so
//everything is all here in the same place

pub fn parse() -> Result<Config> {
    let paths = gather_paths();
    let mut cfg_file = File::open(paths.config_file_path)?;
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;

    let config: Config = toml::from_str(buf.as_str())
        .with_context(|| format!("Unsupported structure for config file."))?;

    Ok(config)
}
