use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::pb::customer_services_server::CustomerServices;
use crate::pb::{
    CreateCustomerRequest, Customer, GetCustomerRequest, GetCustomerResponse, ListCustomerRequest,
    ListCustomerResponse, UpdateCustomerRequest,
};

use super::services::CustomerService;
use super::services::CustomerServiceImpl;

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

        let services = CustomerServiceImpl::new(self.session.clone());

        let customer = services.create(request).await.map(|e| e.into());

        if customer.is_err() {
            return Err(Status::failed_precondition("failed to create a customer"));
        }

        Ok(Response::new(customer.unwrap()))
    }

    async fn update(
        &self,
        request: Request<UpdateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        let request = request.into_inner();

        let services = CustomerServiceImpl::new(self.session.clone());

        let customer = services.update(request).await.map(|e| e.into());

        if customer.is_err() {
            return Err(Status::failed_precondition("failed to update a customer."));
        }

        Ok(Response::new(customer.unwrap()))
    }

    async fn get(
        &self,
        request: Request<GetCustomerRequest>,
    ) -> Result<Response<GetCustomerResponse>, Status> {
        let id = request.into_inner().id;

        let services = CustomerServiceImpl::new(self.session.clone());

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
        request: Request<ListCustomerRequest>,
    ) -> Result<Response<ListCustomerResponse>, Status> {
        let request = request.into_inner();

        let services = CustomerServiceImpl::new(self.session.clone());

        let customers = services.list(request).await.map(|e| {
            let c = e.into_iter().map(|e| e.into()).collect::<_>();

            ListCustomerResponse { customers: c }
        });

        if customers.is_err() {
            return Err(Status::failed_precondition("failed to list customers"));
        }

        Ok(Response::new(customers.unwrap()))
    }
}
