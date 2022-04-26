use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::pb::customer_services_server::CustomerServices;
use crate::pb::{
    CreateCustomerRequest, Customer, GetCustomerRequest, GetCustomerResponse, ListCustomerRequest,
    ListCustomerResponse, UpdateCustomerRequest,
};

use super::services::CustomerService;

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

        let services = super::services::CustomerServiceImpl::new(self.session.clone());

        let customer = services.create(request).await.map(|e| e.into());

        if customer.is_err() {
            return Err(Status::failed_precondition("failed to create a customer"));
        }

        Ok(Response::new(customer.unwrap()))
    }

    async fn update(
        &self,
        _request: Request<UpdateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        todo!()
    }

    async fn get(
        &self,
        request: Request<GetCustomerRequest>,
    ) -> Result<Response<GetCustomerResponse>, Status> {
        let id = request.into_inner().id;

        let services = super::services::CustomerServiceImpl::new(self.session.clone());

        let customer = services.get(id as i64).await.map(|s| s.map(|e| e.into()));

        if customer.is_err() {
            return Err(Status::failed_precondition("failed to get a customer."));
        }

        Ok(Response::new(GetCustomerResponse {
            customer: customer.unwrap(),
        }))
    }

    async fn list(
        &self,
        _request: Request<ListCustomerRequest>,
    ) -> Result<Response<ListCustomerResponse>, Status> {
        todo!()
    }
}
