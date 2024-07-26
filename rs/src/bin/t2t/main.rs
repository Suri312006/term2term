use clap::Parser;
use t2t::{Cli, Handlers, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut handlers = Handlers::new().await?;
    cli.run(&mut handlers).await
}
