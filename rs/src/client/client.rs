use tonic::transport::Channel;

use crate::{
    grpc::{
        convo_service_client::ConvoServiceClient, msg_service_client::MsgServiceClient,
        user_service_client::UserServiceClient,
    },
    Result,
};

// pub struct Handlers {
//     pub user: UserHandler,
//     pub msg: MsgHandler,
//     pub convo: ConvoHandler,
// }

pub struct Handlers {
    pub user: UserServiceClient<Channel>,
    pub convo: ConvoServiceClient<Channel>,
    pub msg: MsgServiceClient<Channel>,
}

//
pub(super) const SERVER_ADDRESS: &str = "http://[::]:50051";
// this shit needs to be paid for :sob:
// pub(super) const SERVER_ADDRESS: &str = "http://t2tserver.fly.dev:54321";

impl Handlers {
    pub async fn new() -> Result<Self> {
        // Ok(Handlers {
        //     //TODO: create wrappers around these so you can have convenience functions
        //     user: UserHandler::new().await?,
        //     msg: MsgHandler::new().await?,
        //     convo: ConvoHandler::new().await?,
        // })
        Ok(Handlers {
            user: UserServiceClient::connect(SERVER_ADDRESS).await?,
            convo: ConvoServiceClient::connect(SERVER_ADDRESS).await?,
            msg: MsgServiceClient::connect(SERVER_ADDRESS).await?,
        })
    }
}
