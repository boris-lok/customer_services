use crate::pb::customer_services_server::CustomerServices;
use crate::pb::{
    CreateCustomerRequest, Customer, GetCustomerRequest, GetCustomerResponse, ListCustomerRequest,
    ListCustomerResponse, UpdateCustomerRequest,
};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tonic::{Request, Response, Status};

use super::json::table::Customers;

#[derive(Debug)]
pub struct CustomerServicesImpl {
    connection_pool: Arc<Pool<Postgres>>,
}

impl CustomerServicesImpl {
    pub fn new(connection_pool: Arc<Pool<Postgres>>) -> Self {
        Self { connection_pool }
    }
}

#[tonic::async_trait]
impl CustomerServices for CustomerServicesImpl {
    async fn get(
        &self,
        request: Request<GetCustomerRequest>,
    ) -> Result<Response<GetCustomerResponse>, Status> {
        let id: u64 = request.into_inner().id;

        let sql = Query::select()
            .columns(vec![
                Customers::Id,
                Customers::Name,
                Customers::Email,
                Customers::Phone,
                Customers::CreatedAt,
                Customers::UpdatedAt,
            ])
            .from(Customers::Table)
            .and_where(Expr::col(Customers::Id).eq(id))
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        let customer = sqlx::query_as::<_, super::json::customer::Customer>(&sql)
            .fetch_optional(&*self.connection_pool)
            .await;

        dbg!(&customer);

        let message = GetCustomerResponse {
            customer: customer.ok().flatten().map(|e| e.into()),
        };

        Ok(Response::new(message))
    }

    async fn create(
        &self,
        request: Request<CreateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        todo!()
    }

    async fn update(
        &self,
        request: Request<UpdateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        todo!()
    }

    async fn list(
        &self,
        request: Request<ListCustomerRequest>,
    ) -> Result<Response<ListCustomerResponse>, Status> {
        todo!()
    }
}
