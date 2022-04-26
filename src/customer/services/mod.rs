use async_trait::async_trait;
use common::utils::alias::AppResult;
use sqlx::{Pool, Postgres};

use crate::pb::CreateCustomerRequest;

use super::{
    json::customer::Customer,
    repo::{postgres_repo::CustomerRepoImpl, CustomerRepo},
};

#[async_trait]
pub trait CustomerService {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>>;

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer>;
}

pub struct CustomerServiceImpl {
    pool: Pool<Postgres>,
}

impl CustomerServiceImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CustomerService for CustomerServiceImpl {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>> {
        let repo = CustomerRepoImpl;

        repo.get(id, &self.pool.clone()).await
    }

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer> {
        let repo = CustomerRepoImpl;

        let mut tx = self.pool.begin().await.unwrap();

        let customer = repo.create(request, &mut *tx).await;

        let _ = tx.commit().await.unwrap();

        customer
    }
}
