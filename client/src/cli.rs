use crate::{Args, Commands};

use std::io;

use t2t::initialize as init;

pub fn run(args: Args) {
    match args.cmd {
        Commands::Init {} => initialize(),
        Commands::Send { message, recepient } => println!("Sending {} to {}", message, recepient),

        Commands::List {
            conversations,
            friends,
            users,
            notifications,
        } => println!("Listing!"),
        Commands::Search {
            messages,
            friends,
            users,
        } => println!("Searching!"),
    }
}

pub fn initialize() {
    println!("Starting Initialziation.");

    let mut username = String::new();

    let paths = init::gather_paths();

    match init::check_existing_config(&paths).unwrap() != true {
        true => {
            println!("What name would you like?");

            io::stdin()
                .read_line(&mut username)
                .expect("Error reading user input.");

            init::initialize(username).unwrap();
            println!(
                "Config file written to {}",
                paths.config_file_path.to_str().unwrap()
            );
        }
        false => {
            println!("Found existing config file.");

            init::initialize(username).unwrap();
        }
    }

    println!("Initialization Success!")
}
