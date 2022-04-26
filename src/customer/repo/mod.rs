use crate::customer::json::customer::Customer;
use crate::pb::CreateCustomerRequest;
use crate::utils::alias::PostgresAcquire;
use async_trait::async_trait;
use common::utils::alias::AppResult;

pub mod postgres_repo;

#[async_trait]
pub trait CustomerRepo {
    async fn get(
        &self,
        id: i64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Option<Customer>>;

    async fn create(
        &self,
        request: CreateCustomerRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Customer>;
}
