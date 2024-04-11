
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // short and long just means that theres a -m, --message
    #[arg(short, long)]
    message: String,

    // recipient
    #[arg(short, long)]
    recipient: String,
}


fn main() {

    let cli = Cli::parse();

    // handle parsing / configuration options
    let args = client::Args{
       message: cli.message,
       recipient: cli.recipient,
    };

    client::run(args);
}
