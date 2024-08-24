use std::env;

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn new() -> Self {
        // ignoring error here
        let _ = dotenvy::dotenv();
        Config {
            db_url: env::var("DB_URL").unwrap(),
        }
    }
}
