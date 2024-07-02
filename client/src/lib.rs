use core::fmt;
use std::{
    fs::{create_dir, File},
    io::{self, ErrorKind, Write},
    str::FromStr,
};

use xdg_home::home_dir;

pub fn initialize() {
    let err_msg = "Unable to find your HOME directory. \
        Consider setting $HOME to your home directory and try again.";

    let home_path = home_dir().expect(err_msg);

    println!("{}", home_path.to_str().expect("Bad unicode in HOME path."));

    let config_dir_path = String::from_str(home_path.to_str().expect("Something went wrong"))
        .unwrap()
        + "/.config/term2term";

    let config_file_path = config_dir_path.clone() + "/config.toml";

    // if the file already exists, we dont wanna overwrite it

    match File::open(&config_file_path) {
        Ok(_) => {
            // ok so the file already exists
            println!("Found existing config file!");
        }
        Err(err) => {
            // file doesnt exist, we need to create the default one

            println!("{}", &config_dir_path);
            match create_dir(&config_dir_path) {
                Ok(()) => {}
                Err(err) => {
                    if err.kind() != ErrorKind::AlreadyExists {
                        eprintln!("Something went wrong with creating directory for config file.")
                    }
                }
            }

            println!("Starting Initialziation. Please answer the following questions.");

            let mut config_file = File::create(&config_file_path)
                .expect("Something went wrong with creating default config file.");

            let mut username = String::new();

            println!("What name would you like?");
            io::stdin()
                .read_line(&mut username)
                .expect("Error reading user input.");

            //TODO: Themes? and yeah thats kind of everything we need

            #[rustfmt::skip]
            let config_file_contents = format!(
                "theme = \"Default\"
[User] 
name = \"{}\"
key = \"\"",
                username.trim()
            );

            config_file
                .write_all(config_file_contents.as_bytes())
                .expect("Something went wrong writing default config file.");
        }
    }
}
