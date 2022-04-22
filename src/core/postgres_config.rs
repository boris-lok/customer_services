#[derive(Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connection: u32,
}

impl PostgresConfig {
    /// Config the postgres connection info from env.
    pub fn new() -> Self {
        let host = dotenv::var("POSTGRES_HOST").expect("Can read postgres host from env.");

        let database =
            dotenv::var("POSTGRES_DATABASE").expect("Can read postgres database from env.");

        let username =
            dotenv::var("POSTGRES_USERNAME").expect("Can read postgres username from env.");

        let password =
            dotenv::var("POSTGRES_PASSWORD").expect("Can read postgres password from env.");

        let port = dotenv::var("POSTGRES_PORT")
            .expect("Can read the port from env.")
            .parse::<u16>()
            .expect("Can parse the port to u16");

        let max_connection = dotenv::var("POSTGRES_MAX_CONNECTION")
            .expect("Can read the postgres max connection from env.")
            .parse::<u32>()
            .expect("Can parse the max_connection to u8");

        Self {
            host,
            port,
            username,
            password,
            database,
            max_connection,
        }
    }
}
