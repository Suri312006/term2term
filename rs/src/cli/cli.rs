#![allow(unused)] // just for convenience, can remove later

use super::args::{Commands, ConversationArgs, MessageVariants, SearchVariants, UserArgs};
use crate::Result;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// A quick way to send a message into the current conversation, if it exists.
    message: Option<String>,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Init {} => handle_init(),
            // Commands::Send { message, recepient } => send(message, recepient),
            Commands::Message(msg_args) => match msg_args.command {
                MessageVariants::Send { message } => {
                    // let msg = Message::new(message)?;
                    //
                    // msg.send()?;
                    //
                    // println!("{}", "Message sent!".green());
                    Ok(())
                }
                MessageVariants::List { all } => {
                    todo!()
                }
            },

            Commands::Search(search_args) => match search_args.command {
                SearchVariants::Messages { query } => todo!(),
                SearchVariants::Friends { query } => todo!(),
                SearchVariants::Users { query } => todo!(),
            },
            Commands::Conversation(convo_args) => handle_convo(convo_args),
            Commands::User(user_args) => handle_user(user_args),
        }
    }
}

fn handle_init() -> Result<()> {
    // println!("{}", "Starting Initialziation.".green());
    //
    // let paths = Paths::new()?;
    //
    // match Config::check_existing()? != true {
    //     true => {
    //         println!("What name would you like?");
    //         let mut username = String::new();
    //
    //         io::stdin()
    //             .read_line(&mut username)
    //             .expect("Error reading user input.");
    //
    //         // create user and write defualt config
    //         let new_user = User::new(&username.trim())?;
    //
    //         let mut default_cfg = Config::default();
    //
    //         default_cfg.users.push(new_user);
    //
    //         default_cfg.write()?;
    //
    //         // Config::write_default(&new_user)?;
    //
    //         println!(
    //             "Config file written to {}",
    //             paths.config_file_path.to_str().unwrap()
    //         );
    //     }
    //     false => {
    //         println!("{}", "Found existing config file.".green());
    //
    //         // verify that user is real
    //         if User::curr()?.verify()? != true {
    //             println!(
    //                 "{}",
    //                 //TODO: would be nice to write the file
    //                 "Corrupted Config Detected.\nPlease delete config file and run `t2t init`"
    //                     .red()
    //             );
    //
    //             return Ok(());
    //         }
    //     }
    // }
    //
    // println!("{}", "Initialization Success!".green());
    // Ok(())
    todo!()
}

fn handle_user(user_args: UserArgs) -> Result<()> {
    // if user_args.list == true {
    //     todo!("List users")
    // }
    //
    // match user_args.command {
    //     Some(args) => match args {
    //         UserVariants::New { username } => {
    //             // create new user
    //             let user = User::new(&username)?;
    //
    //             let mut config = Config::read()?;
    //
    //             config.users.push(user);
    //
    //             config.write()?;
    //         }
    //
    //         UserVariants::Switch { username } => {
    //             let cfg = Config::read()?;
    //             match username {
    //                 Some(username) => match cfg.users.iter().find(|user| user.name == username) {
    //                     Some(user) => {
    //                         let mut state = State::read()?;
    //                         state.user = Some(user.clone());
    //                         state.write()?;
    //                     }
    //
    //                     None => {
    //                         bail!("User not found, be sure to use an exact name as argument.")
    //                     }
    //                 },
    //
    //                 // fuzzy find for users
    //                 None => {
    //                     //TODO: please do this, supposed to be fuzzy finding
    //                     let cfg = Config::read()?;
    //                 }
    //             }
    //         }
    //     },
    //
    //     // should just be useless
    //     None => {}
    // };
    //
    // Ok(())
    todo!()
}

fn handle_convo(lol: ConversationArgs) -> Result<()> {
    // // make the api request
    // //
    // // then display it
    // match lol.command {
    //     ConversationVariants::List => {
    //         let convos = Convo::list_for(User::curr()?)?;
    //
    //         println!("{:#?}", convos);
    //     }
    //     ConversationVariants::Select => {
    //         todo!()
    //     }
    //     ConversationVariants::Start => {
    //         let users = User::find(None)?;
    //
    //         let mut input = String::new();
    //
    //         println!("Type the numper of the user you would like to communicate with.");
    //         for (i, user) in users.iter().enumerate() {
    //             println!("{}. {}", i, user.name);
    //         }
    //
    //         io::stdin()
    //             .read_line(&mut input)
    //             .with_context(|| "unable to read line from conversation selector")?;
    //
    //         //TODO: dont have to crach here, just have user reinput
    //         let selection = input
    //             .trim()
    //             .parse::<usize>()
    //             .with_context(|| "unable to parse int")?;
    //
    //         // make the api call to create convo
    //         println!("selecting {}", selection);
    //         let other_user = users
    //             .get(selection)
    //             .with_context(|| "yo somethign wrong with select")?;
    //
    //         // write selection to config file to parse later ie when they send a new message
    //         let mut state = State::read()?;
    //
    //         state.convo = Some(Convo::new(&User::curr()?, &other_user)?);
    //
    //         state.write()?;
    //     }
    // }
    // Ok(())
    todo!()
}
