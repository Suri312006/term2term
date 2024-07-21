use anyhow::Result;
use clap::{Args, Parser, Subcommand};
/// Welcome to Term2Term, an Easy to Use Terminal Communicator!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// A quick way to send a message into the current conversation, if it exists.
    message: Option<String>,
}

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
    command: SearchVariants,
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
    command: ConversationVariants,
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
    command: MessageVariants,
}

#[derive(Debug, Subcommand)]
pub enum MessageVariants {
    /// Selects the current conversation.
    Send {
        #[arg(short, long, value_parser)]
        message: String,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true, arg_required_else_help(true))]
pub struct UserArgs {
    #[command(subcommand)]
    command: Option<UserVariants>,

    #[arg(short, long, default_value_t = false)]
    list: bool,
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

mod cli;
fn main() -> Result<()> {
    let args = Cli::parse();
    cli::run(args)?;
    Ok(())
}
