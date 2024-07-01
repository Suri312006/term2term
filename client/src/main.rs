use clap::{Parser, Subcommand};
// simple greeter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

//TODO: figure out what args we need for our platform
struct Args {
    //Name of greetee
    // #[arg(short, long)]
    // name: String,
    //
    // //Number of times
    // #[arg(short, long, default_value_t = 1)]
    // count: u64,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

fn main() {
    let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}", args.name);
    // }
}
