use tonic::transport::Channel;

use crate::{
    grpc::{
        convo_service_client::ConvoServiceClient, msg_service_client::MsgServiceClient,
        user_service_client::UserServiceClient,
    },
    Result,
};

use super::user::UserHandler;

pub struct Client {
    pub user_handler: UserHandler,
    pub msg_handler: MsgServiceClient<Channel>,
    pub convo_handler: ConvoServiceClient<Channel>,
}

pub(super) const SERVER_ADDRESS: &str = "http://[::1]:50051";

impl Client {
    pub async fn new() -> Result<Client> {
        Ok(Client {
            //TODO: create wrappers around these so you can have convenience functions
            user_handler: UserHandler::new().await?,
            msg_handler: MsgServiceClient::connect(SERVER_ADDRESS).await?,
            convo_handler: ConvoServiceClient::connect(SERVER_ADDRESS).await?,
        })
    }
}
