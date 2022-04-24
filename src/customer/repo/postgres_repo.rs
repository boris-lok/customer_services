use std::sync::Arc;

use async_trait::async_trait;
use sea_query::{Expr, PostgresQueryBuilder};
use sea_query::Query;
use sqlx::{Pool, Postgres};

use common::utils::alias::AppResult;
use common::utils::error::AppError;

use crate::customer::json::customer::Customer;
use crate::customer::json::table::Customers;
use crate::customer::repo::repo::CustomerRepo;
use crate::pb::CreateCustomerRequest;

#[derive(Debug, Clone)]
pub struct PostgresCustomerRepo {
	connection_pool: Arc<Pool<Postgres>>,
}

impl PostgresCustomerRepo {
	pub fn new(connection_pool: Arc<Pool<Postgres>>) -> Self {
		Self { connection_pool }
	}
}

#[async_trait]
impl CustomerRepo for PostgresCustomerRepo {
	async fn get(&self, id: i64) -> AppResult<Option<Customer>> {
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

		let customer = sqlx::query_as::<_, Customer>(&sql)
			.fetch_optional(&*self.connection_pool)
			.await
			.map_err(|e| AppError::DatabaseError(e.to_string()));

		customer
	}

	async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer> {
		use chrono::Utc;
		let name = request.name.clone().into();
		let email = request.email.into();
		let phone = request.phone.into();

		let cols: Vec<Customers> = vec![Customers::Id, Customers::Name, Customers::Email, Customers::Phone, Customers::CreatedAt];

		let sql = Query::insert()
			.into_table(Customers::Table)
			.columns(cols.clone())
			.values_panic(vec!["1".into(), name, email, phone, Utc::now().into()])
			.returning(Query::select()
				.columns(cols)
				.take())
			.to_string(PostgresQueryBuilder);

		dbg!(&sql);

		let customer = sqlx::query_as::<_, Customer>(&sql)
			.fetch_one(&*self.connection_pool)
			.await
			.map_err(|e| AppError::DatabaseError(e.to_string()));

		customer
	}
}