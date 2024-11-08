use color_eyre::eyre::{eyre, Result};
use log::error;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct Db {}

impl Db {
    pub async fn connect(conn_url: &str) -> Result<Pool<Postgres>> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(conn_url)
            .await
            .map_err(|err| {
                error!("{err}");
                eyre!(err)
            })
    }
}
