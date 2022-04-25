use sea_query::{Expr, PostgresQueryBuilder};
use sea_query::Query;
use sqlx::{Acquire, PgConnection, Postgres, Transaction};

use common::utils::alias::AppResult;
use common::utils::error::AppError;

use crate::customer::json::customer::Customer;
use crate::customer::json::table::Customers;
use crate::pb::CreateCustomerRequest;

use async_trait::async_trait;
use crate::customer::repo::CustomerRepo;
use crate::utils::alias::PostgresAcquire;

pub struct CustomerRepoImpl;


/// references: https://qiita.com/FuJino/items/08b4c3298918191eab65

#[async_trait]
impl CustomerRepo for CustomerRepoImpl {
	async fn get(&self, id: i64, executor: impl PostgresAcquire<'_> + 'async_trait) -> AppResult<Option<Customer>> {
		let mut conn = executor.acquire().await.unwrap();

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

		sqlx::query_as::<_, Customer>(&sql)
			.fetch_optional(&mut *conn)
			.await
			.map_err(|e| AppError::DatabaseError(e.to_string()))
	}
}

/*
impl PostgresCustomerRepo {
    pub async fn create(
        request: CreateCustomerRequest,
        conn: &mut PgConnection,
    ) -> AppResult<Customer> {
        use chrono::Utc;

        let name = request.name.clone().into();
        let email = request.email.into();
        let phone = request.phone.into();

        let cols: Vec<Customers> = vec![
            Customers::Id,
            Customers::Name,
            Customers::Email,
            Customers::Phone,
            Customers::CreatedAt,
        ];

        let sql = Query::insert()
            .into_table(Customers::Table)
            .columns(cols.clone())
            .values_panic(vec!["1".into(), name, email, phone, Utc::now().into()])
            .returning(Query::select().columns(cols).take())
            .to_string(PostgresQueryBuilder);

        dbg!(&sql);

        sqlx::query_as::<_, Customer>(&sql)
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
*/