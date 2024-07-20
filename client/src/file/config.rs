use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Write},
    result::Result::Ok,
};

use anyhow::{anyhow, Context};
use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::core::user::User;

use super::paths::Paths;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub theme: Theme,
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Theme {
    Default,
}

impl Config {
    fn read() -> Result<Config> {
        let paths = Paths::new()?;
        let mut cfg_file = File::open(paths.config_file_path)?;
        let mut buf = String::new();
        cfg_file.read_to_string(&mut buf)?;

        let config: Config = toml::from_str(buf.as_str())
            .with_context(|| "Unsupported structure for config file.")?;

        Ok(config)
    }

    pub fn check_existing() -> Result<bool> {
        let paths = Paths::new()?;
        match File::open(paths.config_file_path.clone()) {
            Ok(_) => Ok(true),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    return Ok(false);
                }

                Err(anyhow!(err.to_string()))
            }
        }
    }

    pub fn write_default(user: &User) -> Result<()> {
        let default_cfg = Config {
            theme: Theme::Default,
            user: user.clone(),
        };

        let paths = Paths::new()?;

        // check if aleady exists? overwrite existing config
        let mut cfg_file = match OpenOptions::new()
            .truncate(true)
            .write(true)
            .read(true)
            .open(paths.config_file_path.clone())
        {
            Ok(file) => Ok(file),

            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(File::create(paths.state_file_path.clone()).unwrap()),
                _ => Err(anyhow!(
                    "there was an error trying to write defualt config file {}",
                    err.to_string()
                )),
            },
        }?;

        let file_data = toml::to_string(&default_cfg)?;
        cfg_file.write_all(file_data.as_bytes())?;

        Ok(())
    }
}
