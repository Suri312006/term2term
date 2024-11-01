use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub auth_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        // ignoring error here
        let _ = dotenvy::dotenv();
        Config {
            db_url: env::var("DATABASE_URL").unwrap(),
            auth_secret: env::var("AUTH_SECRET").unwrap(),
        }
    }
}
