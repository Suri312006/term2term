use color_eyre::eyre::Result;
use t2t_server::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = Config::new();

    println!("{conf:#?}");

    Ok(())
}
