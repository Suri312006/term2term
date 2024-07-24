use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Write},
};

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

use super::{config::ConfigUser, Paths};


#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigConvo {
    // somethign about users
    pub id: String,
    pub user1_id: String,
    pub user2_id: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct State {
    pub convo: Option<ConfigConvo>,
    pub user: Option<ConfigUser>,
}

impl State {
    pub fn read() -> Result<State> {
        todo!()
    }

    pub fn write(&mut self) -> Result<()> {
        let paths = Paths::new()?;

        // opens and wipes file
        let mut state_f = match OpenOptions::new()
            .truncate(true)
            .write(true)
            .read(true)
            .open(paths.state_file_path.clone())
        {
            Ok(file) => Ok(file),

            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(File::create(paths.state_file_path.clone()).unwrap()),
                _ => Err(Error::from(
                    format!("there was an error trying to write state {}",
                    err.to_string()
                ))),
            },
        }?;

        let state_data = toml::to_string(self)?;
        state_f.write_all(state_data.as_bytes())?;
        Ok(())
    }
}

// creates new state and writes file
fn create_state() -> Result<State> {
    let paths = Paths::new()?;

    let mut state_f = File::create(paths.state_file_path)?;

    let state = State {
        convo: None,
        user: None,
    };

    let file_data = toml::to_string(&state)?;

    state_f.write_all(file_data.as_bytes())?;

    Ok(state)
}
