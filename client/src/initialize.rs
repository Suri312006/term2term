use std::{
    fs::{create_dir, File},
    io::{ErrorKind, Write},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    api::{self, verify_user},
    config
};

use super::Paths;

use keyring::Entry;

use anyhow::{anyhow, Context, Result};

use xdg_home::home_dir;

pub fn gather_paths() -> Paths {
    let err_msg = "Unable to find your HOME directory. \
        Consider setting $HOME to your home directory and try again.";

    let home_path = home_dir().expect(err_msg);

    let config_dir_path = PathBuf::from(
        String::from_str(home_path.to_str().unwrap()).unwrap() + "/.config/term2term",
    );

    let config_file_path: PathBuf = [config_dir_path.to_str().unwrap(), "config.toml"]
        .iter()
        .collect();

    Paths {
        config_dir_path,
        config_file_path,
    }
}

pub fn initialize(username: String) -> Result<()> {
    // check if the config flie already exists
    // if it doesnt, remove the file
    let paths = gather_paths();
    if !check_existing_config(&paths)? {
        match create_dir(&paths.config_dir_path) {
            Ok(()) => {}
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists {
                    return Err(anyhow!(
                        "Weird error while creating directory. {}",
                        err.to_string()
                    ));
                }
            }
        }

        let new_user = api::register_new_user(&username)?;

        let entry = Entry::new("term2term", new_user.name.as_str()).unwrap();

        let _ = entry.set_password(&new_user.id).unwrap();

        let mut config_file = File::create(&paths.config_file_path)?;

        #[rustfmt::skip]
            let default_cfg = format!(
                "theme = \"Default\"
[user] 
# Do Not Change These Values Manually.                    
name = \"{}\"
id = \"{}\"",
                username, "dont worry about it"
            );

        config_file
            .write_all(default_cfg.as_bytes())
            .with_context(|| format!("Unable to write to config file."))?;
    } else {
        let config = config::parse(paths)?;
        let verified = verify_user(config.user)?;

        if !verified {
            return Err(anyhow!(
                "User information is not correct, need to repair account"
            ));
        }
    }

    Ok(())
}

pub fn check_existing_config(paths: &Paths) -> Result<bool, std::io::Error> {
    match File::open(paths.config_file_path.clone()) {
        Ok(_) => Ok(true),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Ok(false);
            }

            Err(err)
        }
    }
}
