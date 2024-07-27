#![allow(unused)] // just for cOnvenience, can remove later

use std::{
    io::{self, stdin, stdout, Write},
    ops::Deref,
    time::{SystemTime, UNIX_EPOCH},
};

use super::args::{
    Commands, ConversationArgs, MessageVariants, SearchVariants, UserArgs, UserVariants,
};
use crate::{
    app,
    args::{ConversationVariants, MessageArgs, SearchArgs},
    files::{
        config::{self, Config, ConfigUser},
        Paths,
    },
    grpc::{
        self, Convo, ConvoList, ListConvoReq, Msg, MsgSendReq, NewConvoReq, NewUserReq,
        Participants, SearchUserReq, User,
    },
    AppState, Error, Handlers, Result,
};

use clap::{CommandFactory, Parser};
use colored::Colorize;
use tokio::sync::watch;
use tonic::{IntoRequest, Request};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// A quick way to send a message into the current conversation, if it exists.
    #[arg(short, long)]
    message: Option<String>,
}

impl Cli {
    pub async fn run(self, app: &mut AppState) -> Result<()> {
        if let Some(message) = self.message {
            handle_message(
                MessageArgs {
                    command: MessageVariants::Send { message },
                },
                app,
            )
            .await?;
            return Ok(());
        } else if self.command.is_none() {
            Cli::command().print_help();
            return Ok(());
        }

        match self.command.unwrap() {
            Commands::Init {} => handle_init(app).await,
            Commands::Message(msg_args) => handle_message(msg_args, app).await,
            Commands::Search(search_args) => handle_search(search_args, app).await,
            Commands::Conversation(convo_args) => handle_convo(convo_args, app).await,
            Commands::User(user_args) => handle_user(user_args, app).await,
        }
    }
}

async fn handle_init(app: &mut AppState) -> Result<()> {
    println!("{}", "Starting Initialziation.".green());

    match Config::check_existing(&app.paths)? {
        // already exists
        true => {
            println!("{}", "Found existing config file.".green());

            println!("{}", "Verifying Config File".green());
            for user in app.config.users.clone() {
                let res = app
                    .handlers
                    .user
                    .verify(<ConfigUser as Into<User>>::into(user.clone()).into_request())
                    .await?
                    .into_inner();

                if !res.verified {
                    println!(
                        "{}",
                        format!("WARNING: User not verified {:?}, could be malformed?", user).red()
                    );
                }
            }
        }

        false => {
            println!("What name would you like?");
            let mut username = String::new();

            io::stdin()
                .read_line(&mut username)
                .expect("Error reading user input.");

            // create user and write defualt config
            let user = app
                .handlers
                .user
                .create(
                    NewUserReq {
                        username: username.trim().to_string(),
                    }
                    .into_request(),
                )
                .await?
                .into_inner();

            app.config.users.push(user.clone().into());
            app.cache.user = Some(user.into());

            app.cache.write(&app.paths);
            app.config.write(&app.paths);

            println!(
                "Config file written to {}",
                &app.paths
                    .config_file_path
                    .to_str()
                    .ok_or(Error::from("unable to parse config file path into string"))?
            );
        }
    }

    Ok(())
}

async fn handle_user(user_args: UserArgs, app: &mut AppState) -> Result<()> {
    if let Some(cmd) = user_args.command {
        match cmd {
            UserVariants::New { username } => {
                let new_user = app
                    .handlers
                    .user
                    .create(Request::new(NewUserReq { username }))
                    .await?
                    .into_inner();

                app.config.users.push(new_user.clone().into());
                app.cache.user = Some(new_user.into());
                println!("{}", "New User Created!".green());

                app.config.write(&app.paths)?;
                app.cache.write(&app.paths)?;

                Ok(())
            }

            UserVariants::Switch { username } => Ok(todo!()),
        }
    } else {
        Err(Error::from("No user arg passed in."))
    }
}

async fn handle_convo(convo_args: ConversationArgs, app: &mut AppState) -> Result<()> {
    match convo_args.command {
        ConversationVariants::List => {
            let res = app
                .handlers
                .convo
                .list(
                    ListConvoReq {
                        user: Some(app.cache.curr_user()?.into()),
                    }
                    .into_request(),
                )
                .await?
                .into_inner();
            for convo in res.convos {
                println!("{:#?}", convo);
            }
            Ok(())
        }
        ConversationVariants::Start => {
            let all_users = app
                .handlers
                .user
                .search(
                    SearchUserReq {
                        kind: 0,
                        query: None,
                    }
                    .into_request(),
                )
                .await?
                .into_inner();
            println!("Type the number of the user you would like to start a conversation with");

            for (i, user) in all_users.users.clone().into_iter().enumerate() {
                println!("{}: {}", i, user.name);
            }

            let mut buf = String::new();
            let _ = stdout().flush();
            stdin().read_line(&mut buf);

            let index: usize = buf.trim().parse()?;

            let new_convo = app
                .handlers
                .convo
                .create(NewConvoReq {
                    participants: Some(Participants {
                        users: vec![
                            app.cache.curr_user()?.into(),
                            all_users
                                .users
                                .get(index)
                                .ok_or(Error::from("yoooo what the sigma"))
                                .cloned()?,
                        ],
                    }),
                })
                .await?
                .into_inner();

            app.cache.convo = Some(new_convo.try_into()?);
            app.cache.write(&app.paths);

            Ok(())
        }
        ConversationVariants::Select => {
            let convos = app
                .handlers
                .convo
                .list(ListConvoReq {
                    user: Some(app.cache.curr_user()?.into()),
                })
                .await?
                .into_inner()
                .convos;

            println!("Type the number of the conversation you want to switch to");

            for (i, convo) in convos.clone().into_iter().enumerate() {
                println!("{}: {:?}", i, convo.participants);
            }

            let mut buf = String::new();
            let _ = stdout().flush();
            stdin().read_line(&mut buf);

            let index: usize = buf.trim().parse()?;

            app.cache.convo = Some(
                convos
                    .get(index)
                    .ok_or(Error::from("cant find the thing"))?
                    .to_owned()
                    .try_into()?,
            );
            app.cache.write(&app.paths);

            Ok(())
        }
    }
}

async fn handle_search(search_args: SearchArgs, app: &mut AppState) -> Result<()> {
    match search_args.command {
        SearchVariants::Users { query } => todo!(),
        SearchVariants::Friends { query } => todo!(),
        SearchVariants::Messages { query } => todo!(),
    }
}

async fn handle_message(msg_args: MessageArgs, app: &mut AppState) -> Result<()> {
    match msg_args.command {
        MessageVariants::Send { message } => {
            let convo: Convo = app
                .cache
                .convo
                .clone()
                .ok_or(Error::from(
                    "Please select convo first before trying to send a message",
                ))?
                .try_into()?;

            let res = app
                .handlers
                .msg
                .send(
                    MsgSendReq {
                        body: message,
                        convo: Some(
                            app.cache
                                .convo
                                .clone()
                                .ok_or(Error::from(
                                    "Please select convo first before trying to send a message",
                                ))?
                                .try_into()?,
                        ),

                        author: Some(
                            app.cache
                                .user
                                .clone()
                                .ok_or(Error::from("user does not exist in cache"))?
                                .into(),
                        ),
                        unix_time: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    }
                    .into_request(),
                )
                .await?;
            println!("{}", "Message sent!".green());

            Ok(())
        }
        MessageVariants::List { all } => {
            todo!()
        }
    }
}
