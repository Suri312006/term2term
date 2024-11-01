use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::TokenData;
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

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AuthInterceptor {
    auth: Auth,
}

#[async_trait]
impl RequestInterceptor for AuthInterceptor {
    async fn intercept(&self, mut req: Request<BoxBody>) -> Result<Request<BoxBody>, Status> {
        match req.headers().get("authorization").map(|v| v.to_str()) {
            Some(Ok(token)) => {
                let token_data = self
                    .auth
                    .validate_token(token)
                    .map_err(|_err| Status::unauthenticated("Failed to Validate Token"))?

                let user_id_header_val = HeaderValue::from_str(&token_data.claims.sub)
                    .map_err(|_e| Status::internal("Failed to convert user_id to header value"))?;
                req.headers_mut().insert("user_id", user_id_header_val);
                Ok(req)
            }

            _ => Err(Status::unauthenticated("lock in")),
        }
    }
}

#[derive(Clone)]
pub struct Auth {
    config: Config,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,
}

#[derive(Serialize, Deserialize, Debug)]
// these define my authentication scheme
pub struct Claims {
    iss: String, // issuer = hermes
    sub: String, // subject = userid
    iat: usize,  // issued at time
    exp: usize,  // expiry time
}

impl Auth {
    pub fn gen_token(&self, user_id: String) -> Result<String, AuthError> {
        let delta = SystemTime::now()
            .checked_add(Duration::from_secs(60))
            .ok_or(AuthError::TokenCreation)?
            .duration_since(UNIX_EPOCH)
            .map_err(|_err| AuthError::TokenCreation)?
            .as_secs();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_err| AuthError::TokenCreation)?
            .as_secs();

        let c = Claims {
            iss: "Hermes".to_string(),
            sub: user_id,
            iat: now as usize,
            exp: delta as usize,
        };

        let header = jsonwebtoken::Header::default();

        let secret = jsonwebtoken::EncodingKey::from_secret("keep yourself safe".as_bytes());

        let res =
            jsonwebtoken::encode(&header, &c, &secret).map_err(|_e| AuthError::TokenCreation)?;

        Ok(res)
    }

    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, AuthError> {
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);

        let secret = jsonwebtoken::DecodingKey::from_secret("keep yourself safe".as_bytes());

        let res = jsonwebtoken::decode::<Claims>(token, &secret, &validation)
            .map_err(|_e| AuthError::InvalidToken)?;

        println!("{:#?}", res);

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth() {
        let auth = Auth {
            config: Config::default(),
        };
        let x = auth.gen_token("lol".to_string()).unwrap();

        println!("{x}");

        auth.validate_tken(&x).unwrap();
    }
}
