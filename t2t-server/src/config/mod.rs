use std::{env, path::Path};

#[derive(Debug)]
pub struct Config {
    db_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().expect(".env file not found");
        Config {
            db_url: env::var("DB_URL").unwrap(),
        }
    }
}
