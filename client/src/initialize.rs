use std::{
    fs::{create_dir, File},
    io::{ErrorKind, Write},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    api::{self, verify_user},
    config::{self, Config},
};

use super::Paths;

use anyhow::{anyhow, Context, Result};

use xdg_home::home_dir;

pub fn gather_paths() -> Paths {
    let err_msg = "Unable to find your HOME directory. \
        Consider setting $HOME to your home directory and try again.";

    let home_path = home_dir().expect(err_msg);

    let t2t_dir = PathBuf::from(
        String::from_str(home_path.to_str().unwrap()).unwrap() + "/.config/term2term",
    );

    let cfg_fp: PathBuf = [t2t_dir.to_str().unwrap(), "config.toml"].iter().collect();

    let state_fp: PathBuf = [t2t_dir.to_str().unwrap(), "state.toml"].iter().collect();

    Paths {
        t2t_dir_path: t2t_dir,
        config_file_path: cfg_fp,
        state_file_path: state_fp,
    }
}

pub fn initialize(username: String) -> Result<()> {
    // check if the config flie already exists
    // if it doesnt, remove the file
    let paths = gather_paths();
    if !check_existing_config(&paths)? {
        match create_dir(&paths.t2t_dir_path) {
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

        let mut config_file = File::create(&paths.config_file_path)?;

        #[rustfmt::skip]
        // instead of using this string, just serialize the struct
        let new_cfg = Config{
            theme: config::Theme::Default,
            user: new_user,
        };

        let file_data = toml::to_string(&new_cfg)?;

        config_file
            .write_all(file_data.as_bytes())
            .with_context(|| format!("Unable to write to config file."))?;
    } else {
        let config = config::parse_config()?;
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
