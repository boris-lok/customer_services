use common::utils::alias::AppResult;
use crate::core::config::Config;
use crate::core::environment::Environment;

mod customer;

mod pb {
    include!("../gen/grpc.customer.rs");
}

#[tokio::main]
async fn main() -> AppResult<()> {
    Ok(())
}
