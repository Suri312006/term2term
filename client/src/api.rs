use anyhow::Result;

use crate::config::User;


pub fn register_new_user(username: &String) -> User {
    let params = [("name", username.to_string())];
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://t2tserver.fly.dev/user/register")
        // .post("http://localhost:8080/user/register")
        .form(&params)
        .send()
        .unwrap();
    let x: User = res.json().unwrap();

    return x;
}

pub fn verify_user(user: User) -> Result<bool>{
    let params = [("name", user.name.to_string())];
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://t2tserver.fly.dev/user/verify")
        // .post("http://localhost:8080/user/register")
        .form(&params)
        .send()?;

    let x: bool  = res.json()?;
    Ok(x)
}
