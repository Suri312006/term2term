#![allow(unused)] // just for convenience, can remove later

use super::args::{
    Commands, ConversationArgs, MessageVariants, SearchVariants, UserArgs, UserVariants,
};
use crate::{
    app,
    files::{
        config::{self, Config, ConfigUser},
        Paths,
    },
    grpc::NewUserReq,
    AppState, Error, Handlers, Result,
};

use clap::Parser;
use colored::Colorize;
use tonic::Request;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// A quick way to send a message into the current conversation, if it exists.
    message: Option<String>,
}

impl Cli {
    pub async fn run(self, app: &mut AppState) -> Result<()> {
        match self.command {
            Commands::Init {} => handle_init(app),
            // Commands::Send { message, recepient } => send(message, recepient),
            Commands::Message(msg_args) => match msg_args.command {
                MessageVariants::Send { message } => Ok(()),
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
