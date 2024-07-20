use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::SERVER_ROOT;
#[derive(Deserialize, Serialize, Debug, Clone )]
pub struct User {
    pub name: String,
    pub id: String,
}

impl User {
    pub fn new(username: &str) -> Result<User> {
        let params = [("name", username.to_string())];
        let client = reqwest::blocking::Client::new();
        let res = client
            // .post("https://t2tserver.fly.dev/user/register")
            .post(format!("{}{}", SERVER_ROOT, "/user/register"))
            // .post("http://localhost:8080/user/register")
            .form(&params)
            .send()
            .with_context(|| "Something went wrong accessing remote server")?;

        let x: User = res
            .json()
            .with_context(|| "unable to parse json from server")?;

        Ok(x)
    }

    pub fn verify(&self) -> Result<bool> {
        #[derive(Deserialize)]
        struct VerifyCheck {
            verified: bool,
        }

        let params = [("name", self.name.to_string()), ("id", self.id.to_string())];
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("{}{}", SERVER_ROOT, "/user/verify"))
            .form(&params)
            .send()
            .with_context(|| "Something went wrong accessing remote server")?;

        let x: VerifyCheck = res
            .json()
            .with_context(|| "unable to parse json from server")?;

        Ok(x.verified)
    }

    pub fn find(params: Option<Vec<(&str, String)>>) -> Result<Vec<User>> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("{}{}", SERVER_ROOT, "/user"))
            .form(&params)
            .send()
            .with_context(|| "Something went wrong accessing remote server")?;

        let users: Vec<User> = res
            .json()
            .with_context(|| "unable to parse users from go server.")?;

        Ok(users)
    }

}

