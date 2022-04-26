use crate::customer::json::customer::Customer;
use crate::pb::{CreateCustomerRequest, ListCustomerRequest, UpdateCustomerRequest};
use async_trait::async_trait;
use common::utils::alias::{AppResult, PostgresAcquire};

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

    async fn list(
        &self,
        request: ListCustomerRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Vec<Customer>>;

    async fn update(
        &self,
        request: UpdateCustomerRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<bool>;
}
