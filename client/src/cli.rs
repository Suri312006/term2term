use crate::{Cli, Commands, ConversationArgs, ConversationVariants, SearchVariants};
use colored::Colorize;

use std::io::{self};

use anyhow::{Context, Result};
use t2t::{
    api::{create_conversation, find_user, list_conversations},
    config::{self, parse_config},
    initialize::{self as init, gather_paths},
    state::State,
};

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
        Commands::Conversation(convo_args) => handle_convo(convo_args),
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

            init::initialize(username.trim().to_string())?;
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

fn handle_convo(lol: ConversationArgs) -> Result<()> {
    // make the api request
    //
    // then display it
    match lol.command {
        ConversationVariants::List => {
            let convos = list_conversations(parse_config()?.user)?;

            println!("{:#?}", convos);
        }
        ConversationVariants::Select => {
            todo!()
        }
        ConversationVariants::Start => {
            let users = find_user(None)?;

            let mut input = String::new();

            println!("Type the numper of the user you would like to communicate with.");
            for (i, user) in users.iter().enumerate() {
                println!("{}. {}", i, user.name);
            }

            io::stdin()
                .read_line(&mut input)
                .with_context(|| "unable to read line from conversation selector")?;

            //TODO: dont have to crach here, just have user reinput
            let selection = input
                .trim()
                .parse::<usize>()
                .with_context(|| "unable to parse int")?;

            // make the api call to create convo
            println!("selecting {}", selection);
            let other_user = users
                .get(selection)
                .with_context(|| "yo somethign wrong with select")?;

            // write selection to config file to parse later ie when they send a new message
            let mut state = State::read()?;

            state.curr_convo = Some(create_conversation(&parse_config()?.user, &other_user)?);

            state.write()?;
        }
    }
    Ok(())
}
