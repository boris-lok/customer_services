use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use snowflake::SnowflakeGenerator;
use tonic::transport::Server;

use common::configs::config::Config;
use common::configs::id_generator_config::IdGeneratorConfig;
use common::configs::postgres_config::PostgresConfig;
use common::utils::alias::AppResult;
use common::utils::tools::{create_database_connection, tracing_initialize};
use common::utils::tools::create_id_generator;

use crate::customer::handler::CustomerServicesImpl;
use crate::pb::customer_services_server::CustomerServicesServer;

mod customer;
mod utils;

mod pb {
    include!("../gen/grpc.customer.rs");
}

lazy_static! {
    static ref ID_GENERATOR: Arc<Mutex<SnowflakeGenerator>> = {
        let config = IdGeneratorConfig::new();
        let generator = create_id_generator(config);
        Arc::new(Mutex::new(generator))
    };
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let _ = dotenv::from_path("env/dev.env").unwrap();

    let config = Config::new();

    tracing_initialize(config.debug, "logs/", "customers");

    let database_config = PostgresConfig::new();

    let database_connection = create_database_connection(database_config).await.unwrap();

    let customer_service = CustomerServicesImpl::new(database_connection.clone());

    let addr = "127.0.0.1:50001".parse().unwrap();

    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .add_service(CustomerServicesServer::new(customer_service))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}
