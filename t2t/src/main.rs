use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::app::App;

mod action;
mod app;
mod cli;
mod client;
mod components;
mod config;
mod errors;
mod logging;
mod tui;
mod utils;
mod grpc {
    tonic::include_proto!("t2t");
}

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    //TODO: we need a channel to go into this (mpsc)
    //TODO: do we handle file io / everything compared to the app handling it?
    tokio::spawn(async move {});
    //TODO: we need a channel to come out of this (maybe a broadcast?)

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}
