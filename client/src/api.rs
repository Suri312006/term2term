use anyhow::{Context, Ok, Result};
use serde::Deserialize;

use crate::{config::User, state::Conversation};

// const SERVER_ROOT: &str = "https://t2tserver.fly.dev";
const SERVER_ROOT: &str = "http://localhost:8080";

pub fn register_new_user(username: &String) -> Result<User> {
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

#[derive(Deserialize)]
struct VerifyCheck {
    verified: bool,
}

pub fn verify_user(user: User) -> Result<bool> {
    let params = [("name", user.name.to_string()), ("id", user.id.to_string())];
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

pub fn list_conversations(user: User) -> Result<Vec<Conversation>> {
    let params = [("userid", user.id.to_string())];
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("{}{}", SERVER_ROOT, "/convo/list"))
        .form(&params)
        .send()
        .with_context(|| "Something went wrong accessing remote server")?;

    let convos: Vec<Conversation> = res
        .json()
        .with_context(|| "unable to parse conversations from go server.")?;

    Ok(convos)
}

pub fn create_conversation(curr_user: &User, other_user: &User) -> Result<Conversation> {
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

    let convo: Conversation = res.json().with_context(|| "unable to parse conversation")?;

    Ok(convo)
}

pub fn find_user(params: Option<Vec<(&str, String)>>) -> Result<Vec<User>> {
    {
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
