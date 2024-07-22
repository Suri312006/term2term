pub mod t2tproto {
    tonic::include_proto!("t2t");
}

use anyhow::Result;
use t2tproto::{user_service_client::UserServiceClient, VerifyUserReq};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::]:50051").await?;

    let response = client
        .verify_user(Request::new(VerifyUserReq {
            id: "1#16456".to_string(),
            name: "Surednra".to_string(),
        }))
        .await?;

    println!("RESPONSE = {:?}", response.into_inner());

    Ok(())
}
