use async_trait::async_trait;
use common::utils::alias::AppResult;
use crate::customer::json::customer::Customer;
use crate::pb::CreateCustomerRequest;

#[async_trait]
pub trait CustomerRepo {
	async fn get(&self, id: i64) -> AppResult<Option<Customer>>;

	// async fn create(&self, CreateCustomerRequest) -> AppResult<Customer>;
}
