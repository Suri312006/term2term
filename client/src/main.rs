use clap::{Parser, Subcommand};

// simple greeter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
//TODO: figure out what args we need for our platform
pub struct Args {
    #[command(subcommand)]
    cmd: Commands,
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
    List {
        #[arg(short, long)]
        conversations: bool,

        #[arg(short, long)]
        friends: bool,

        #[arg(short, long)]
        users: bool,

        #[arg(short, long)]
        notifications: bool,
    },

    /// Search various aspects within the service.
    Search {
        #[arg(short, long)]
        messages: String,

        #[arg(short, long)]
        friends: String,

        #[arg(short, long)]
        users: String,
    },
}

mod cli;
fn main() {
    cli::run(Args::parse());
}
