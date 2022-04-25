use crate::customer::json::customer::Customer;
use crate::pb::CreateCustomerRequest;
use async_trait::async_trait;
use common::utils::alias::AppResult;

#[async_trait]
pub trait CustomerRepo {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>>;

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer>;
}
