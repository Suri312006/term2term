use std::{net::SocketAddr, str::FromStr};

use color_eyre::{config, eyre::Result};
use hermes::{
    db::Db,
    grpc::{
        auth_service_server::{AuthService, AuthServiceServer},
        user_service_server::UserServiceServer,
    },
    middleware::auth::{Auth, AuthInterceptor},
    services::{AuthServer, UserServer},
    Config,
};
use tonic::{service::Interceptor, transport::Server};
use tonic_middleware::InterceptorFor;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let conf = Config::default();

    let db = Db::connect(conf.db_url.as_str()).await?;

    let auth = Auth::new(conf);

    Server::builder()
        .add_service(AuthServiceServer::new(AuthServer::new(
            db.clone(),
            auth.clone(),
        )))
        .add_service(InterceptorFor::new(
            UserServiceServer::new(UserServer::new(db.clone())),
            AuthInterceptor::new(auth.clone()),
        ))
        .serve(SocketAddr::from_str("[::1]:50051").expect("weird socket addr"))
        .await?;

    Ok(())
}
