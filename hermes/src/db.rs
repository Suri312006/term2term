use color_eyre::eyre::{eyre, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use sqlx::mysql::MySqlPoolOptions;
// etc.

//#[async_std::main] // Requires the `attributes` feature of `async-std`
//#[tokio::main]
//// or #[actix_web::main]
//async fn main() -> Result<(), sqlx::Error> {
//    // Create a connection pool
//    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
//    //  for SQLite, use SqlitePoolOptions::new()
//    //  etc.
//    let pool = PgPoolOptions::new()
//        .max_connections(5)
//        .connect("postgres://postgres:password@localhost/test")
//        .await?;
//
//    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
//    let row: (i64,) = sqlx::query_as("SELECT $1")
//        .bind(150_i64)
//        .fetch_one(&pool)
//        .await?;
//
//    assert_eq!(row.0, 150);
//
//    Ok(())
//}

pub struct Db {}

impl Db {
    pub async fn connect(conn_url: &str) -> Result<Pool<Postgres>> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(conn_url)
            .await
            .map_err(|err| eyre!(err))
    }
}
