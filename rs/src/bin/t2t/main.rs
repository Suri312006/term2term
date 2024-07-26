use clap::Parser;
use t2t::{AppState, Cli, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut app = AppState::instantiate().await?;
    cli.run(&mut app).await
}
