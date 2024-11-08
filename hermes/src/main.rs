use color_eyre::eyre::Result;
use hermes::{Config, Hermes};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let conf = Config::default();

    let hermes = Hermes::new(conf).await?;

    hermes.run().await
}
