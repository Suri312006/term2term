use log::error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};

use crate::Config;

#[derive(Clone)]
pub struct Authenticator {
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
    pub iss: String, // issuer = hermes
    pub sub: String, // subject = userid
    pub iat: usize,  // issued at time
    pub exp: usize,  // expiry time
}

impl Authenticator {
    pub fn new(config: Config) -> Self {
        Authenticator { config }
    }
    pub fn gen_token(&self, user_id: &str) -> Result<String, AuthError> {
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
            sub: user_id.to_string(),
            iat: now as usize,
            exp: delta as usize,
        };

        let header = jsonwebtoken::Header::default();

        let secret = jsonwebtoken::EncodingKey::from_secret(self.config.auth_secret.as_bytes());

        let res = jsonwebtoken::encode(&header, &c, &secret).map_err(|err| {
            error!("{err}");
            AuthError::TokenCreation
        })?;

        Ok(res)
    }

    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, AuthError> {
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);

        let secret = jsonwebtoken::DecodingKey::from_secret(self.config.auth_secret.as_bytes());

        let res = jsonwebtoken::decode::<Claims>(token, &secret, &validation).map_err(|err| {
            error!("{err}");
            AuthError::InvalidToken
        })?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: i need to somehow test network delay but i got no clue lol, seems like an integration
    //test
    #[test]
    fn auth() {
        let auth = Authenticator {
            config: Config::default(),
        };
        let x = auth.gen_token("lol").unwrap();

        println!("{x}");

        auth.validate_token(&x).unwrap();
    }
}
