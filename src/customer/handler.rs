use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::pb::customer_services_server::CustomerServices;
use crate::pb::{
    CreateCustomerRequest, Customer, GetCustomerRequest, GetCustomerResponse, ListCustomerRequest,
    ListCustomerResponse, UpdateCustomerRequest 
};

use super::uow::UnitOfWork;
use common::utils::alias::AppResult;

pub struct CustomerServicesImpl {
    uow: Arc<UnitOfWork>,
}

impl CustomerServicesImpl {
    pub fn new(uow: Arc<UnitOfWork>) -> Self {
        Self { uow }
    }
}

#[tonic::async_trait]
impl CustomerServices for CustomerServicesImpl {
    async fn create(
        &self,
        request: Request<CreateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        let request = request.into_inner();

        let bt = self.uow.begin_transaction().await;
        if bt.is_ok() {
            let customer: AppResult<super::json::customer::Customer> = self.uow.repo.create(request).await;

            let _ = self.uow.commit().await;

            if customer.is_err() {
                let _ = self.uow.rollback().await.unwrap();
            }
            
            return Ok(Response::new(customer.unwrap().into()));
        }

        Err(Status::failed_precondition("Database error."))
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
        let c = self.uow.repo.get(id as i64).await.unwrap();

        let message = GetCustomerResponse {
            customer: c.map(|e| e.into()),
        };

        Ok(Response::new(message))
    }

    async fn list(
        &self,
        request: Request<ListCustomerRequest>,
    ) -> Result<Response<ListCustomerResponse>, Status> {
        todo!()
    }
}
