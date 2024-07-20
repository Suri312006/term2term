use std::{
    fs::{create_dir, File},
    io::{ErrorKind, Write},
};

use anyhow::{anyhow, Context, Result};

use crate::{core::user::User, file::{config::Config, paths::Paths}};

pub fn initialize(username: String) -> Result<()> {
    // check if the config flie already exists
    // if it doesnt, remove the file
    let paths = Paths::new()?;
    if Config::check_existing()? {
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

        let new_user = User::new(&username);

        Config::write_default();

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
