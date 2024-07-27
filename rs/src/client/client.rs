use tonic::transport::Channel;

use crate::grpc::{
    convo_service_client::ConvoServiceClient, msg_service_client::MsgServiceClient,
    user_service_client::UserServiceClient,
};

use color_eyre::Result;

#[derive(Debug)]
pub struct Handlers {
    pub user: UserServiceClient<Channel>,
    pub convo: ConvoServiceClient<Channel>,
    pub msg: MsgServiceClient<Channel>,
}

//NOTE: using fly for this might cost money, would be more fun to host our own?
pub(super) const SERVER_ADDRESS: &str = "http://[::]:50051";

impl Handlers {
    pub async fn new() -> Result<Self> {
        Ok(Handlers {
            user: UserServiceClient::connect(SERVER_ADDRESS).await?,
            convo: ConvoServiceClient::connect(SERVER_ADDRESS).await?,
            msg: MsgServiceClient::connect(SERVER_ADDRESS).await?,
        })
    }
}
