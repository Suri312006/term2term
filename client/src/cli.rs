use crate::{Cli, Commands, ListVariants, SearchVariants};

use std::io;

use anyhow::{Ok, Result};
use t2t::initialize as init;

pub fn run(args: Cli) -> Result<()> {
    match args.command {
        Commands::Init {} => initialize(),
        Commands::Send { message, recepient } => send(message, recepient),

        Commands::List(listArgs) => match listArgs.command {
            ListVariants::Conversations => Ok(()),
            ListVariants::Friends => Ok(()),
            ListVariants::Users => Ok(()),
            ListVariants::Notifications => Ok(()),
        },
        Commands::Search(searchArgs) => match searchArgs.command {
            SearchVariants::Messages { query } => Ok(()),
            SearchVariants::Friends { query } => Ok(()),
            SearchVariants::Users { query } => Ok(()),
        },
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
