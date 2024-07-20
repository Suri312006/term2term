use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    fs::{remove_file, File, OpenOptions},
    io::{ErrorKind, Read, Write},
    result::Result::Ok,
};

use crate::initialize::gather_paths;

#[derive(Deserialize, Serialize, Debug)]
pub struct State {
    pub curr_convo: Option<Conversation>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Conversation {
    // somethign about users
    pub id: String,
    pub user1_id: String,
    pub user2_id: String,
}

impl State {
    pub fn read() -> Result<State> {
        let paths = gather_paths();
        match File::open(&paths.state_file_path) {
            Ok(mut state_f) => {
                let mut buf = String::new();
                state_f.read_to_string(&mut buf);

                let state: State = match toml::from_str(buf.as_str()) {
                    Ok(state) => state,
                    Err(_) => {
                        // we just want it to remove, no matter what
                        let _ = remove_file(paths.state_file_path);
                        println!("{}", "Found Bad State File, Wiping...".red());
                        create_state()?
                    }
                };

                Ok(state)
            }

            Err(err) => match err.kind() {
                ErrorKind::NotFound => create_state(),
                _ => Err(anyhow!(err.to_string())),
            },
        }
    }

    pub fn write(&mut self) -> Result<()> {
        let paths = gather_paths();

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
                _ => Err(anyhow!(
                    "there was an error trying to write state {}",
                    err.to_string()
                )),
            },
        }?;

        let state_data = toml::to_string(self).with_context(|| "error parsing current state")?;
        state_f.write_all(state_data.as_bytes())?;
        Ok(())
    }
}

// creates new state and writes file
fn create_state() -> Result<State> {
    let paths = gather_paths();

    let mut state_f = File::create(paths.state_file_path)?;

    let state = State { curr_convo: None };

    let file_data = toml::to_string(&state)?;

    state_f
        .write_all(file_data.as_bytes())
        .with_context(|| "err writing to state file")?;

    Ok(state)
}
