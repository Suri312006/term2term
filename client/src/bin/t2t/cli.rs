use crate::{
    Cli, Commands, ConversationArgs, ConversationVariants, MessageVariants, SearchVariants,
};
use colored::Colorize;
use t2t::{
    core::{
        convo::Convo,
        message::{self, Message},
        user::User,
    },
    file::{
        config::{self, Config},
        paths::Paths,
        state::State,
    },
};

use std::io::{self};

use anyhow::{Context, Ok, Result};

pub fn run(args: Cli) -> Result<()> {
    match args.command {
        Commands::Init {} => init(),
        // Commands::Send { message, recepient } => send(message, recepient),
        Commands::Message(msg_args) => match msg_args.command {
            MessageVariants::Send { message } => {
                let msg = Message::new(message)?;

                msg.send()?;

                println!("{}", "Message sent!".green());
                Ok(())
            }
        },

        Commands::Search(search_args) => match search_args.command {
            SearchVariants::Messages { query } => Ok(()),
            SearchVariants::Friends { query } => Ok(()),
            SearchVariants::Users { query } => Ok(()),
        },
        Commands::Conversation(convo_args) => handle_convo(convo_args),
    }
}

fn init() -> Result<()> {
    println!("{}", "Starting Initialziation.".green());

    let paths = Paths::new()?;

    match Config::check_existing()? != true {
        true => {
            println!("What name would you like?");
            let mut username = String::new();

            io::stdin()
                .read_line(&mut username)
                .expect("Error reading user input.");

            // create user and write defualt config
            let new_user = User::new(&username.trim())?;
            Config::write_default(&new_user)?;

            println!(
                "Config file written to {}",
                paths.config_file_path.to_str().unwrap()
            );
        }
        false => {
            println!("{}", "Found existing config file.".green());

            // verify that user is real
            if Config::read()?.user.verify()? != true {
                println!(
                    "{}",
                    //TODO: would be nice to write the file
                    "Corrupted Config Detected.\nPlease delete config file and run `t2t init`"
                        .red()
                );

                return Ok(());
            }
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
            let convos = Convo::list_for(User::curr()?)?;

            println!("{:#?}", convos);
        }
        ConversationVariants::Select => {
            todo!()
        }
        ConversationVariants::Start => {
            let users = User::find(None)?;

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

            state.curr_convo = Some(Convo::new(&User::curr()?, &other_user)?);

            state.write()?;
        }
    }
    Ok(())
}
