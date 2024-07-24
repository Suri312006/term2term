
use clap::Parser;
use t2t::{cli::cli::Cli, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()
}
