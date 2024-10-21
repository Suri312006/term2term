use clap::Parser;
use color_eyre::Result;
use t2t::{AppState, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let mut app = AppState::instantiate().await?;
    cli.run(&mut app).await
}
