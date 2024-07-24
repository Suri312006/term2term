use clap::Parser;
use t2t::{Cli, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut client = Client::new().await?;
    cli.run(&mut client).await
}
