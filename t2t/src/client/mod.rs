mod commands;

use color_eyre::eyre::Result;
pub use commands::*;
mod handlers;
use handlers::*;
use tokio::sync::mpsc;

pub struct Client {
    handlers: Handlers,
    command_rx: mpsc::UnboundedReceiver<Command>,
    response_tx: mpsc::UnboundedSender<Command>,
}

impl Client {
    // run this inside a tokio spawn
    pub async fn new(
        reciever: mpsc::UnboundedReceiver<Command>,
        responder: mpsc::UnboundedSender<Command>,
    ) -> Result<Self> {
        Ok(Client {
            handlers: Handlers::connect().await?,
            command_rx: reciever,
            response_tx: responder,
        })
    }

    /// run inside tokio task
    pub async fn run(&mut self) -> Result<()> {
        // loop over every msg and handle that shit
        panic!("PENIS");
        while let Some(msg) = self.command_rx.recv().await {
            todo!();
        }
        Ok(())
    }
}

// what should i do
