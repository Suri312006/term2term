use crate::{Args, Commands};

use std::io;

use anyhow::{Ok, Result};
use t2t::initialize as init;

pub fn run(args: Args) -> Result<()> {
    match args.cmd {
        Commands::Init {} => initialize(),
        Commands::Send { message, recepient } => send(message, recepient),

        Commands::List {
            conversations,
            friends,
            users,
            notifications,
        } => list(conversations, friends, users, notifications),
        Commands::Search {
            messages,
            friends,
            users,
        } => search(messages, friends, users),
    }
}

fn initialize() -> Result<()> {
    println!("Starting Initialziation.");

    let mut username = String::new();

    let paths = init::gather_paths();

    match init::check_existing_config(&paths).unwrap() != true {
        true => {
            println!("What name would you like?");

            io::stdin()
                .read_line(&mut username)
                .expect("Error reading user input.");

            init::initialize(username.trim().to_string()).unwrap();
            println!(
                "Config file written to {}",
                paths.config_file_path.to_str().unwrap()
            );
        }
        false => {
            println!("Found existing config file.");

            init::initialize(username)?;
        }
    }

    println!("Initialization Success!");
    Ok(())
}

fn send(message: String, recepient: String) -> Result<()> {
    todo!()
}

fn list(conversations: bool, friends: bool, users: bool, notifications: bool) -> Result<()> {
    todo!()
}
fn search(messages: String, friends: String, users: String) -> Result<()> {
    todo!()
}
