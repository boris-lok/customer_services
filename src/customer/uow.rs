use common::utils::alias::AppResult;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use super::repo::postgres_repo::PostgresCustomerRepo;
use super::repo::repo::CustomerRepo;

#[derive(Clone)]
pub struct UnitOfWork {
    connection_pool: Arc<Pool<Postgres>>,
    pub repo: Arc<Box<dyn CustomerRepo + Sync + Send>>,
}

impl UnitOfWork {
    pub fn new(connection_pool: Arc<Pool<Postgres>>) -> Self {
        let repo = PostgresCustomerRepo::new(Arc::clone(&connection_pool));

        Self {
            connection_pool,
            repo: Arc::new(Box::new(repo)),
        }
    }

    pub async fn begin_transaction(&self) -> AppResult<()> {
        Ok(())
    }

    pub async fn commit(&self) -> AppResult<()> {
        Ok(())
    }

    pub async fn rollback(&self) -> AppResult<()> {
        Ok(())
    }
}
