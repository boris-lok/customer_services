use crate::core::postgres_config::PostgresConfig;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};

/// Create a database connection.
pub async fn create_database_connection(config: PostgresConfig) -> Pool<Postgres> {
    let connection_options = PgConnectOptions::new()
        .host(&config.host)
        .database(&config.database)
        .username(&config.username)
        .password(&config.password)
        .port(config.port);

    PgPoolOptions::new()
        .max_connections(config.max_connection)
        .connect_with(connection_options)
        .await
        .unwrap()
}
