use std::fmt::format;

use anyhow::{anyhow, Context, Result};
use colored::Colorize;

use crate::SERVER_ROOT;

use super::{convo::Convo, user::User};

pub struct Message {
    // author: User,
    // recipient: User,
    convo: Convo,
    body: String,
    // read: bool,
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
            .post(format!("{}{}", SERVER_ROOT, "/msg/send"))
            .form(&params)
            .send()
            .with_context(|| "server crashed")?;

        if res.status() != 200 {
            return Err(anyhow!("Error from server."));
        };

        Ok(())
    }
}
