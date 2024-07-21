use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::SERVER_ROOT;

use super::{convo::Convo, user::User};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    // author: User,
    // recipient: User,
    convo: Convo,
    body: String,
    // read: bool,
}

pub enum MessageSearch {
    All,
    Unread,
}

impl Message {
    pub fn new(body: String) -> Result<Message> {
        Ok(Message {
            // author: User::curr()?,
            // recipient,
            convo: Option::expect(
                Convo::curr()?,
                format!("{}", "Must select a conversation first.".red()).as_str(),
            ),
            body,
            // read: false,
        })
    }

    pub fn send(&self) -> Result<()> {
        let params = [
            ("author_id", self.convo.user1_id.as_str()),
            ("recipient_id", self.convo.user2_id.as_str()),
            ("convo_id", self.convo.id.as_str()),
            ("body", self.body.as_str()),
        ];

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("{}{}", SERVER_ROOT, "/msg"))
            .form(&params)
            .send()
            .with_context(|| "server crashed")?;

        if res.status() != 200 {
            return Err(anyhow!("Error from server."));
        };

        Ok(())
    }

    pub fn search(args: MessageSearch) -> Result<Vec<Message>> {
        let user = User::curr()?;
        let mut params = HashMap::new();
        params.insert("user_id", user.id.as_str());

        match args {
            MessageSearch::All => params.insert("all", "true"),
            MessageSearch::Unread => params.insert("unread", "true"),
        };

        let client = reqwest::blocking::Client::new();

        let res = client
            .get(format!("{}{}", SERVER_ROOT, "/msg"))
            .form(&params)
            .send()
            .with_context(|| "server error when searching for message")?;
        let messages: Vec<Message> = res
            .json()
            .with_context(|| "unable to parse messages from server")?;

        Ok(messages)
    }
}
