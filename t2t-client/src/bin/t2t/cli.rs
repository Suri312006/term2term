#![allow(unused)] // just for convenience

use clap::{Args, Command, CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// A quick way to send a message into the current conversation, if it exists.
    #[arg(short, long)]
    message: Option<String>,
}


#[derive(Subcommand, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub enum Commands {
    /// Initializes your Term2Term account.
    Init {},

    /// Message related things.
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
    /// search for messages
    Messages {
        #[arg(short, long)]
        query: String,
    },

    /// search for friends
    Friends {
        #[arg(short, long)]
        query: String,
    },

    /// search for Term2Term users
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

    /// View all conversations
    List,

    // start a new conversation!
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

    /// List Unread Messages
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
    /// Create a new user
    New {
        #[arg(short, long)]
        username: String,
    },

    /// switch to another account
    Switch {
        #[arg(short, long)]
        username: Option<String>,
    },

    ChangeName {
        #[arg(short, long)]
        newname: String,
    },
}
