#![allow(unused)] // just for convenience, can remove later

use clap::{Args, Parser, Subcommand};


#[derive(Subcommand, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub enum Commands {
    /// Initializes your Term2Term account.
    Init {},

    Message(MessageArgs),

    /// Search various aspects within the service.
    Search(SearchArgs),

    /// Conversation related things.
    Conversation(ConversationArgs),

    /// User related things
    User(UserArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct SearchArgs {
    #[command(subcommand)]
    pub command: SearchVariants,
}

#[derive(Debug, Subcommand)]
pub enum SearchVariants {
    Messages {
        #[arg(short, long)]
        query: String,
    },

    Friends {
        #[arg(short, long)]
        query: String,
    },

    Users {
        #[arg(short, long)]
        query: String,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true, visible_alias = "con")]
pub struct ConversationArgs {
    #[command(subcommand)]
    pub command: ConversationVariants,
}

#[derive(Debug, Subcommand)]
pub enum ConversationVariants {
    /// Selects the current conversation.
    Select,
    List,
    Start,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true, visible_alias = "msg")]
pub struct MessageArgs {
    #[command(subcommand)]
    pub command: MessageVariants,
}

#[derive(Debug, Subcommand)]
pub enum MessageVariants {
    /// Sends message into previously selected convo
    Send {
        #[arg(short, long, value_parser)]
        message: String,
    },

    List {
        #[arg(short, long, default_value_t = false)]
        all: bool,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true, arg_required_else_help(true))]
pub struct UserArgs {
    #[command(subcommand)]
    pub command: Option<UserVariants>,

    #[arg(short, long, default_value_t = false)]
    pub list: bool,
}

#[derive(Debug, Subcommand)]
pub enum UserVariants {
    New {
        #[arg(short, long)]
        username: String,
    },

    Switch {
        #[arg(short, long)]
        username: Option<String>,
    },
}
