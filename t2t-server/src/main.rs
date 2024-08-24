use color_eyre::eyre::Result;
use t2t_server::Config;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let conf = Config::new();
    println!("{conf:#?}");

    Ok(())
}
