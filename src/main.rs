use tonic::transport::Server;

use common::configs::postgres_config::PostgresConfig;
use common::utils::alias::AppResult;
use common::utils::tools::create_database_connection;

use crate::customer::handler::CustomerServicesImpl;
use crate::pb::customer_services_server::CustomerServicesServer;

mod customer;
mod utils;

mod pb {
    include!("../gen/grpc.customer.rs");
}

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::from_path("config/dev.env");

    let postgres = PostgresConfig::new();

    let database_connection = create_database_connection(postgres).await.unwrap();

    let customer_service = CustomerServicesImpl::new(database_connection.clone());

    let addr = "127.0.0.1:50001".parse().unwrap();

    Server::builder()
        .add_service(CustomerServicesServer::new(customer_service))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}
