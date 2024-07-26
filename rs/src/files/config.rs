use std::{
    fs::{create_dir, File, OpenOptions},
    io::{ErrorKind, Read, Write},
};

use serde::{Deserialize, Serialize};

use super::Paths;
use crate::{Error, Result};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ConfigUser {
    pub username: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Config {
    pub theme: Theme,
    pub users: Vec<ConfigUser>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub enum Theme {
    #[default]
    Default,
}

impl Config {
    pub fn read(paths: &Paths) -> Result<Config> {
        let mut cfg_file = match File::open(&paths.config_file_path) {
            Ok(file) => file,
            // file probably didnt exist, just return default and call it a day
            Err(_) => return Ok(Config::default()),
        };

        let mut buf = String::new();
        cfg_file.read_to_string(&mut buf)?;

        let config: Config = toml::from_str(buf.as_str())?;

        Ok(config)
    }

    pub fn check_existing(paths: &Paths) -> Result<bool> {
        match File::open(&paths.config_file_path.clone()) {
            Ok(_) => Ok(true),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    return Ok(false);
                }

                Err(Error::from(err.to_string()))
            }
        }
    }

    //TODO: i would really like to use toml_edit to do this so comments and stuff
    //are preserved but it is what it is you know
    pub fn write(&self, paths: &Paths) -> Result<()> {
        match create_dir(&paths.t2t_dir_path) {
            Ok(()) => {}
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists {
                    return Err(Error::from(format!(
                        "Weird error while creating directory. {}",
                        err.to_string(),
                    )));
                }
            }
        };

        let mut cfg_file = match OpenOptions::new()
            .truncate(true)
            .write(true)
            .read(true)
            .open(paths.config_file_path.clone())
        {
            Ok(file) => Ok(file),

            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(File::create(paths.config_file_path.clone()).unwrap()),
                _ => Err(Error::from(format!(
                    "there was an error trying to write defualt config file {}",
                    err.to_string()
                ))),
            },
        }?;

        let file_data = toml::to_string(&self)?;
        cfg_file.write_all(file_data.as_bytes())?;
        Ok(())
    }
}
