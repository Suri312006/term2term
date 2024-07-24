use tonic::transport::Channel;

use crate::{
    grpc::{
        convo_service_client::ConvoServiceClient, msg_service_client::MsgServiceClient,
        user_service_client::UserServiceClient,
    },
    Result,
};

pub struct Client {
    pub user_handler: UserServiceClient<Channel>,
    pub msg_handler: MsgServiceClient<Channel>,
    pub convo_handler: ConvoServiceClient<Channel>,
}

const SERVER_ADDRESS: &str = "http://[::1]:50051";

impl Client {
    pub async fn new() -> Result<Client> {
        Ok(Client {
            user_handler: UserServiceClient::connect(SERVER_ADDRESS).await?,
            msg_handler: MsgServiceClient::connect(SERVER_ADDRESS).await?,
            convo_handler: ConvoServiceClient::connect(SERVER_ADDRESS).await?,
        })
    }
}
