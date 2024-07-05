use anyhow::Result;
use clap::{Args, Parser, Subcommand};

// simple greeter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initializes your Term2Term account.
    Init {},

    /// Send a message.
    Send {
        #[arg(short, long)]
        message: String,

        #[arg(short, long)]
        recepient: String,
    },

    /// List various aspects within the service.
    List(ListArgs),

    /// Search various aspects within the service.
    Search(SearchArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ListArgs {
    #[command(subcommand)]
    command: ListVariants,
}

#[derive(Debug, Subcommand)]
pub enum ListVariants {
    Conversations,

    Friends,

    Users,

    Notifications,
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

mod cli;
fn main() -> Result<()> {
    let args = Cli::parse();
    cli::run(args)?;

    Ok(())
}
