use tonic::{async_trait, body::BoxBody, Status};

//NOTE: Need to use this specific import
use tonic::codegen::http::{HeaderValue, Request};

use tonic_middleware::RequestInterceptor;

use super::authenticator::Authenticator;

#[derive(Clone)]
pub struct AuthInterceptor {
    auth: Authenticator,
}

impl AuthInterceptor {
    pub fn new(auth: Authenticator) -> Self {
        AuthInterceptor { auth }
    }
}

#[async_trait]
impl RequestInterceptor for AuthInterceptor {
    async fn intercept(&self, mut req: Request<BoxBody>) -> Result<Request<BoxBody>, Status> {
        match req.headers().get("authorization").map(|v| v.to_str()) {
            Some(Ok(token)) => {
                let token_data = self
                    .auth
                    .validate_token(token)
                    .map_err(|_err| Status::unauthenticated("Failed to Validate Token"))?;

                let user_id_header_val = HeaderValue::from_str(&token_data.claims.sub)
                    .map_err(|_e| Status::internal("Failed to convert user_id to header value"))?;
                req.headers_mut().insert("user_id", user_id_header_val);
                Ok(req)
            }

            _ => Err(Status::unauthenticated("lock in")),
        }
    }
}
