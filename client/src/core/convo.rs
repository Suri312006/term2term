use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::SERVER_ROOT;

use super::user::User;
#[derive(Deserialize, Serialize, Debug)]
pub struct Convo {
    // somethign about users
    pub id: String,
    pub user1_id: String,
    pub user2_id: String,
}

impl Convo {
    pub fn new(curr_user: &User, other_user: &User) -> Result<Convo> {
        let params = [
            ("user1_id", curr_user.id.to_string()),
            ("user2_id", other_user.id.to_string()),
        ];

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("{}{}", SERVER_ROOT, "/convo"))
            .form(&params)
            .send()
            .with_context(|| "server crashed")?;

        let convo: Convo = res.json().with_context(|| "unable to parse conversation")?;

        Ok(convo)
    }

    pub fn list_for(user: User) -> Result<Vec<Convo>> {
        let params = [("userid", user.id.to_string())];
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("{}{}", SERVER_ROOT, "/convo/list"))
            .form(&params)
            .send()
            .with_context(|| "Something went wrong accessing remote server")?;

        let convos: Vec<Convo> = res
            .json()
            .with_context(|| "unable to parse conversations from go server.")?;

        Ok(convos)
    }
}
