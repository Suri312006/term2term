#![allow(unused)] // just for convenience, can remove later

pub mod t2tgrpc {
    tonic::include_proto!("t2t");
}

use t2tgrpc::{user_service_client::UserServiceClient, NewUserReq, User};

use t2t::prelude::*;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::]:50051").await?;

    let response = client
        .new_user(Request::new(NewUserReq {
            username: "Surendra".to_string(),
        }))
        .await?;

    println!("RESPONSE = {:?}", response);

    Ok(())
}
