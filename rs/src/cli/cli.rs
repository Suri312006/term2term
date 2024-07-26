#![allow(unused)] // just for convenience, can remove later

use std::time::{SystemTime, UNIX_EPOCH};

use super::args::{
    Commands, ConversationArgs, MessageVariants, SearchVariants, UserArgs, UserVariants,
};
use crate::{
    app,
    files::{
        config::{self, Config, ConfigUser},
        Paths,
    },
    grpc::{Msg, MsgSendReq, NewUserReq},
    AppState, Error, Handlers, Result,
};

use clap::{CommandFactory, Parser};
use colored::Colorize;
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
            print!("{}", message);
            return Ok(());
        } else if self.command.is_none() {
            Cli::command().print_help();
            return Ok(());
        }

        match self.command.unwrap() {
            Commands::Init {} => handle_init(app),
            // Commands::Send { message, recepient } => send(message, recepient),
            Commands::Message(msg_args) => match msg_args.command {
                MessageVariants::Send { message } => {
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
                                .into()
                            ),
                            unix_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                        }
                        .into_request(),
                    ).await?;

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
            Commands::User(user_args) => handle_user(user_args, app).await,
        }
    }
}

fn handle_init(app: &mut AppState) -> Result<()> {
    println!("{}", "Starting Initialziation.".green());
    todo!()
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
                println!("{}", "New User Created!".green());

                app.cache.user = Some(new_user.into());
                app.config.write(&app.paths);
                app.cache.write(&app.paths);

                Ok(())
            }

            UserVariants::Switch { username } => Ok(todo!()),
        }
    } else {
        Err(Error::from("No user arg passed in."))
    }
}

fn handle_convo(lol: ConversationArgs) -> Result<()> {
    todo!()
}
