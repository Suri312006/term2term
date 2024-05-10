use std::os;

use clap::Parser;
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // short and long just means that theres a -m, --message
    #[arg(short, long)]
    message: String,

    // recipient
    #[arg(short, long)]
    recipient: String,

    #[arg(short, long, default_value_t = false)]
    use_server: bool,
}

fn main() {
    dotenv().ok();

    let server_address = std::env::var("PUBLIC_SERVER_URL").expect("PUBLIC_SERVER_URL was not found in server.");

    let cli = Cli::parse();

    // handle parsing / configuration options
    let args = client::Args {
        message: cli.message,
        recipient: cli.recipient,
        server_address: if cli.use_server {
            server_address
        } else {
            String::from("http://localhost:8080/")
        },
    };

    client::run(args);
}
