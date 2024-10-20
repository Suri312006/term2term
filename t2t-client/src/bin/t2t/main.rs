use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;

mod cli;
fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = Cli::parse();
    match opts.command {

    }



    println!("Hello World");
    Ok(())
}
