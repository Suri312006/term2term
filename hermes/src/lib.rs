mod config;
use std::{net::SocketAddr, str::FromStr};

use color_eyre::eyre::{eyre, Result};
pub use config::*;
use db::Db;
use grpc::{
    auth_service_server::AuthServiceServer, convo_service_server::ConvoServiceServer,
    msg_service_server::MsgServiceServer, user_service_server::UserServiceServer,
};
use log::error;
use log::info;
use middleware::auth::{AuthInterceptor, Authenticator};
use services::{AuthServer, ConversationServer, MessageServer, UserServer};
use tonic::transport::{server::Router, Server};
use tonic_middleware::InterceptorFor;
use utils::Log;
pub mod db;
pub mod entities;
pub mod middleware;
pub mod services;
pub mod utils;
pub mod grpc {
    tonic::include_proto!("t2t");
}

pub struct Hermes {
    router: Router,
}

impl Hermes {
    pub async fn new(config: Config) -> Result<Self> {
        Log::init()?;

        //WARN: this shit is erroring with test containers man and idk why
        let db = Db::connect(&config.db_url).await.map_err(|err| {
            error!("{err}");
            err
        })?;

        let auth = Authenticator::new(config);

        let router = Server::builder()
            .add_service(AuthServiceServer::new(AuthServer::new(
                db.clone(),
                auth.clone(),
            )))
            .add_service(InterceptorFor::new(
                UserServiceServer::new(UserServer::new(db.clone())),
                AuthInterceptor::new(auth.clone()),
            ))
            .add_service(InterceptorFor::new(
                MsgServiceServer::new(MessageServer::new(db.clone())),
                AuthInterceptor::new(auth.clone()),
            ))
            .add_service(InterceptorFor::new(
                ConvoServiceServer::new(ConversationServer::new(db.clone())),
                AuthInterceptor::new(auth.clone()),
            ));

        Ok(Hermes { router })
    }

    pub async fn run(self) -> Result<()> {
        info!("Server Running");
        self.router
            .serve(SocketAddr::from_str("[::1]:50051").expect("weird socket addr"))
            .await
            .map_err(|err| eyre!(err))
    }
}
