use crate::core::config::Config;
use crate::core::environment::Environment;

type AppResult<T> = anyhow::Result<T>;

mod core;
mod customer;

mod pb {
    include!("../gen/grpc.customer.rs");
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let config = Config::new();
    let environment = Environment::new(config);

    dbg!(&environment);

    Ok(())
}
