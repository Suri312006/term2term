use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Write},
};

use serde::{Deserialize, Serialize};

use super::{config::ConfigUser, Paths};

use crate::{Error, Result};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigConvo {
    pub id: String,
    pub participants: Option<Vec<ConfigUser>>,
    pub created_at: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cache {
    pub convo: Option<ConfigConvo>,
    pub user: Option<ConfigUser>,
}

impl Cache {
    pub fn read(paths: &Paths) -> Result<Cache> {
        let cache = match OpenOptions::new()
            .truncate(false)
            .write(false)
            .read(true)
            .open(&paths.cache_file_path)
        {
            Ok(mut file) => {
                // file already exists so uhhh, what now??
                let mut file_buf = String::new();
                file.read_to_string(&mut file_buf)?;
                let cache: Cache = ron::from_str(file_buf.as_str())?;
                cache
            }
            Err(err) => {
                if ErrorKind::NotFound == err.kind() {
                    create_cache(paths)?
                } else {
                    return Err(Error::from("Something went wrong when reading from cache."));
                }
            }
        };
        Ok(cache)
    }

    pub fn write(&self, paths: &Paths) -> Result<()> {
        // opens and wipes file
        let mut cache_f = match OpenOptions::new()
            .truncate(true)
            .write(true)
            .read(true)
            .open(&paths.cache_file_path)
        {
            Ok(file) => Ok(file),

            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(File::create(&paths.cache_file_path).unwrap()),
                _ => Err(Error::from(format!(
                    "there was an error trying to write cache {}",
                    err
                ))),
            },
        }?;

        let ron_data = ron::to_string(&self)?;

        cache_f.write_all(ron_data.as_bytes())?;
        Ok(())
    }

    pub fn curr_user(&self) -> Result<ConfigUser> {
        self.user.clone().ok_or(Error::from("No user in cache."))
    }
}

fn create_cache(paths: &Paths) -> Result<Cache> {
    let mut cache_f = File::create(&paths.cache_file_path)?;

    let cache = Cache {
        convo: None,
        user: None,
    };

    let file_data = ron::to_string(&cache)?;

    cache_f.write_all(file_data.as_bytes())?;

    Ok(cache)
}
