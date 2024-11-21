use crate::grpc::{
    auth_service_client::AuthServiceClient, convo_service_client::ConvoServiceClient,
    msg_service_client::MsgServiceClient, user_service_client::UserServiceClient,
};
use color_eyre::eyre::Result;
use tonic::transport::Channel;
static SERVER_URL: &str = "http://[::1]:50051";
impl Handlers {
    pub async fn connect() -> Result<Self> {
        Ok(Handlers {
            message_handler: MsgServiceClient::connect(SERVER_URL).await?,
            user_handler: UserServiceClient::connect(SERVER_URL).await?,
            auth_handler: AuthServiceClient::connect(SERVER_URL).await?,
            convo_handler: ConvoServiceClient::connect(SERVER_URL).await?,
        })
    }
}

pub struct Handlers {
    message_handler: MsgServiceClient<Channel>,
    user_handler: UserServiceClient<Channel>,
    auth_handler: AuthServiceClient<Channel>,
    convo_handler: ConvoServiceClient<Channel>,
}
