
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // message
    #[arg(short, long)]
    message: String,

    // recipient
    #[arg(short, long)]
    recipient: String,
    // count? remove later
    //
    // #[arg(short, long, default_value_t=1)]
    // count: u8,
}


fn main() {
    let cli = Cli::parse();
    println!("{}", cli.recipient);
    println!("{}", cli.message);

    let args = client::Args{
       message: cli.message,
       recipient: cli.recipient,
    };

    


    client::run(args);
    
}
