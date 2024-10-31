use std::{net::SocketAddr, str::FromStr};

use color_eyre::eyre::Result;
use hermes::{
    db::Db,
    services::{user_service_server::UserServiceServer, UserServer},
    Config,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let conf = Config::new();
    let db = Db::connect("lol").await?;

    //Server::builder()
    //    .add_service(UserServiceServer::new(UserServer { }))
    //    .serve(SocketAddr::from_str("[::1]:50051").expect("weird socket addr"))
    //    .await?;

    Ok(())
}
