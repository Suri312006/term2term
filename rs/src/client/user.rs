use tonic::transport::Channel;
use tonic::Request;

use crate::files::config::ConfigUser;
use crate::grpc::user_service_client::UserServiceClient;
use crate::grpc::{NewUserReq, User};
use crate::Result;

use super::SERVER_ADDRESS;

impl From<ConfigUser> for User {
    fn from(value: ConfigUser) -> Self {
        User {
            id: value.id,
            name: value.username,
        }
    }
}

pub struct UserClient {
    client: UserServiceClient<Channel>,
}

impl UserClient {
    pub async fn new() -> Result<Self> {
        Ok(UserClient {
            client: UserServiceClient::connect(SERVER_ADDRESS).await?,
        })
    }

    pub async fn new_user(&mut self, req: NewUserReq) -> Result<User> {
        let req = Request::new(req);

        let res = self.client.create(req).await?;

        println!("RESPONSE: {:?}", &res);
        Ok(res.into_inner())
    }
}
