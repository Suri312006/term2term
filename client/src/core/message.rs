use anyhow::{anyhow, Context, Result};

use crate::{api::SERVER_ROOT, config::User, state::Conversation};

struct Message {
    author: User,
    recipient: User,
    convo: Conversation,
    body: String,
    read: bool,
}

impl Message {
    fn new(author: User, recipient: User, convo: Conversation, body: String) -> Message {
        Message {
            author,
            recipient,
            convo,
            body,
            read: false,
        }
    }

    fn send(&self) -> Result<()> {
        let params = [
            ("author_id", self.author.id.as_str()),
            ("recipient_id", self.recipient.id.as_str()),
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

