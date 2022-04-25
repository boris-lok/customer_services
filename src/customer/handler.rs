use std::ops::DerefMut;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::pb::customer_services_server::CustomerServices;
use crate::pb::{
    CreateCustomerRequest, Customer, GetCustomerRequest, GetCustomerResponse, ListCustomerRequest,
    ListCustomerResponse, UpdateCustomerRequest,
};

use common::utils::alias::AppResult;
use sqlx::{Pool, Postgres};
use tracing::{warn, warn_span};

use super::repo::postgres_repo::PostgresCustomerRepo;

pub struct CustomerServicesImpl {
    session: Pool<Postgres>,
}

impl CustomerServicesImpl {
    pub fn new(session: Pool<Postgres>) -> Self {
        Self { session }
    }
}

#[tonic::async_trait]
impl CustomerServices for CustomerServicesImpl {
    async fn create(
        &self,
        request: Request<CreateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        let request = request.into_inner();

        let mut bt = self.session.clone().begin().await.unwrap();
        let customer = PostgresCustomerRepo::create(request, &mut *bt).await;

        if (bt.commit().await).is_ok() {
            return Ok(Response::new(customer.unwrap().into()));
        }

        Err(Status::failed_precondition("Database query error."))
    }

    async fn update(
        &self,
        request: Request<UpdateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        todo!()
    }

    async fn get(
        &self,
        request: Request<GetCustomerRequest>,
    ) -> Result<Response<GetCustomerResponse>, Status> {
        let id = request.into_inner().id;

        let conn = self.session.clone();
        let c = PostgresCustomerRepo::get(id as i64, &conn).await;

        if let Ok(c) = c {
            let message = GetCustomerResponse {
                customer: c.map(|e| e.into()),
            };

            return Ok(Response::new(message));
        }

        Err(Status::failed_precondition("Database query error."))
    }

    async fn list(
        &self,
        request: Request<ListCustomerRequest>,
    ) -> Result<Response<ListCustomerResponse>, Status> {
        todo!()
    }
}
