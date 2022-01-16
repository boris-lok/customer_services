
type AppResult<T> = anyhow::Result<T>;
type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;

mod core;
mod customer;

#[tokio::main]
async fn main() {

}
