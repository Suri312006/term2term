use super::UserInfo;
pub fn register_new_user(username: &String) -> UserInfo {
    let params = [("username", username.to_string())];
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://t2tserver.fly.dev/user/register")
        // .post("http://localhost:8080/user/register")
        .form(&params)
        .send()
        .unwrap();
    let x: UserInfo = res.json().unwrap();

    return x;
}
