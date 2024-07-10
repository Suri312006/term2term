use crate::{Cli, Commands, ConversationArgs, ConversationVariants, SearchVariants};
use colored::Colorize;

use std::io;

use anyhow::{Ok, Result};
use t2t::{api::list_conversations, config, initialize::{self as init, gather_paths}};

pub fn run(args: Cli) -> Result<()> {
    match args.command {
        Commands::Init {} => initialize(),
        Commands::Send { message, recepient } => send(message, recepient),

        // Commands::List(list_args) => match list_args.command {
        //     ListVariants::Conversations => Ok(()),
        //     ListVariants::Friends => Ok(()),
        //     ListVariants::Users => Ok(()),
        //     ListVariants::Notifications => Ok(()),
        // },
        Commands::Search(search_args) => match search_args.command {
            SearchVariants::Messages { query } => Ok(()),
            SearchVariants::Friends { query } => Ok(()),
            SearchVariants::Users { query } => Ok(()),
        },
        Commands::Conversation(convo_args) => match convo_args.command {
            ConversationVariants::List => {
            },
            ConversationVariants::Select => Ok(()),
        },
    }
}

fn initialize() -> Result<()> {
    println!("{}", "Starting Initialziation.".green());

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

    println!("{}", "Initialization Success!".green());
    Ok(())
}

fn send(message: String, recepient: String) -> Result<()> {
    println!(
        "{} {}{}",
        "Sending Message to".green(),
        recepient.green(),
        "!".green()
    );

    todo!()
}

fn search(messages: String, friends: String, users: String) -> Result<()> {
    todo!()
}

fn conversation(lol: ConversationArgs){

}
