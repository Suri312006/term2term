use color_eyre::eyre::Result;
use t2t_client::{
    user_service_client::{self, UserServiceClient},
    NewUserReq,
};
use tonic::{transport::Channel, IntoRequest};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");

    let mut client: UserServiceClient<Channel> =
        UserServiceClient::connect(String::from("http://[::1]:50051")).await?;

    let res = client
        .create(
            NewUserReq {
                name: "Surendra".to_owned(),
            }
            .into_request(),
        )
        .await?;

    println!("{:#?}", res);

    Ok(())
}
