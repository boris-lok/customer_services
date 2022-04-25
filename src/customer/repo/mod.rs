use async_trait::async_trait;
use common::utils::alias::AppResult;
use crate::customer::json::customer::Customer;
use crate::utils::alias::PostgresAcquire;

pub mod postgres_repo;

#[async_trait]
pub trait CustomerRepo {
	async fn get(&self, id: i64, executor: impl PostgresAcquire<'_> + 'async_trait) -> AppResult<Option<Customer>>;
}