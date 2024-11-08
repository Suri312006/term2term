use std::path::Path;

use color_eyre::Result;
use sqlx::{migrate::Migrator, Connection, PgPool};
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{runners::AsyncRunner, ContainerAsync},
};

pub struct TestDb {}
impl TestDb {
    pub async fn connect() -> Result<(ContainerAsync<Postgres>, String)> {
        let node = Postgres::default().start().await?;

        // prepare connection string
        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432).await?
        );

        let migrate_conn = PgPool::connect(&connection_string).await?;

        Migrator::new(Path::new("./migrations"))
            .await?
            .run(&migrate_conn)
            .await?;

        Ok((node, connection_string))
    }
}
