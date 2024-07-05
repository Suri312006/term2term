use std::{
    fs::{create_dir, File},
    io::{ErrorKind, Write},
    path::PathBuf,
    str::FromStr,
};

use super::Paths;

use serde::Deserialize;

use keyring::Entry;

use anyhow::{anyhow, Context, Result};

#[derive(Deserialize)]
struct UserInfo {
    id: String,
    username: String,
}

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

fn register_new_user(username: &String) -> UserInfo {
    // This will POST a body of `foo=bar&baz=quux`
    let params = [("username", username.to_string())];
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://t2tserver.fly.dev/register")
        .form(&params)
        .send()
        .unwrap();
    let x: UserInfo = res.json().unwrap();

    return x;
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

        let new_user = register_new_user(&username);

        let entry = Entry::new("term2term", new_user.username.as_str()).unwrap();

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
        //TODO: we can validate that the existing user is correct? probably by api call?
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
