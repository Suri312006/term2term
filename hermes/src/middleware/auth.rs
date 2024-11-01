//TODO: https://github.com/Keats/jsonwebtoken
//https://www.shuttle.dev/blog/2024/02/21/using-jwt-auth-rust
//https://medium.com/@cuongta/how-to-encode-and-decode-jwt-using-rust-51f3b757e212
use tonic::{
    async_trait,
    body::BoxBody,
    codegen::http::{HeaderValue, Request, Response},
    IntoRequest, Status,
};
use tonic_middleware::RequestInterceptor;

use crate::{
    grpc::{auth_service_server::AuthService, Token},
    Config,
};

use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct AuthInterceptor {}

#[async_trait]
impl RequestInterceptor for AuthInterceptor {
    async fn intercept(&self, mut req: Request<BoxBody>) -> Result<Request<BoxBody>, Status> {
        match req.headers().get("authorization").map(|v| v.to_str()) {
            Some(Ok(token)) => {
                //let user_id = self
                //    .auth_service
                //    .verify_token(
                //        Token {
                //            access_token: token.to_string(),
                //        }
                //        .into_request(),
                //    )
                //    .await
                //    .map_err(|_| Status::unauthenticated("Failed Authentication"))?
                //    .into_inner()
                //    .id;
                todo!();

                let user_id_header_val = HeaderValue::from_str(&user_id)
                    .map_err(|_e| Status::internal("Failed to convert user_id to header value"))?;
                req.headers_mut().insert("user_id", user_id_header_val);
                Ok(req)
            }

            _ => Err(Status::unauthenticated("lock in")),
        }
    }
}

pub struct Auth {
    config: Config,
}

pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,
}

pub struct Claims {
    username: String,
    exp: usize,
}

impl Auth {
    pub fn verify_token(&self, token: &str) -> Result<bool, AuthError> {
        let token_data = decode::<Claims>(
            bearer
        )
        todo!()
    }
}
