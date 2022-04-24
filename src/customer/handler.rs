use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::customer::repo::repo::CustomerRepo;
use crate::pb::{
	CreateCustomerRequest, Customer, GetCustomerRequest, GetCustomerResponse, ListCustomerRequest,
	ListCustomerResponse, UpdateCustomerRequest,
};
use crate::pb::customer_services_server::CustomerServices;

pub struct CustomerServicesImpl {
	repo: Arc<Box<dyn CustomerRepo + Send + Sync + 'static>>,
}

impl CustomerServicesImpl {
	pub fn new(repo: Arc<Box<dyn CustomerRepo + Send + Sync + 'static>>) -> Self {
		Self { repo }
	}
}

#[tonic::async_trait]
impl CustomerServices for CustomerServicesImpl {
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

	async fn get(
		&self,
		request: Request<GetCustomerRequest>,
	) -> Result<Response<GetCustomerResponse>, Status> {
		let id = request.into_inner().id;
		let c = self.repo.get(id as i64).await.unwrap();
		
		let message = GetCustomerResponse {
			customer: c.map(|e| e.into())
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
