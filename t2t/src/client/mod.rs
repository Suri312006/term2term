mod commands;
use std::sync::mpsc;

pub use commands::*;
use tonic::transport::Channel;

use crate::grpc::{
    auth_service_client::AuthServiceClient, convo_service_client::ConvoServiceClient,
    msg_service_client::MsgServiceClient, user_service_client::UserServiceClient,
};

pub struct Client {
    handlers: Handlers,
    command_rx: mpsc::Receiver<Command>,
    response_tx: mpsc::Sender<Command>,
}

impl Client {
    // run this inside a tokio spawn
    fn new() -> Self {
        todo!();
    }
}

// what should i do
impl Default for Handlers {
    fn default() -> Self {
        todo!();
    }
}
struct Handlers {
    message_handler: MsgServiceClient<Channel>,
    user_handler: UserServiceClient<Channel>,
    auth_handler: AuthServiceClient<Channel>,
    convo_handler: ConvoServiceClient<Channel>,
}
