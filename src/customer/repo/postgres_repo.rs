use sea_query::Query;
use sea_query::{Expr, PostgresQueryBuilder};

use common::utils::alias::AppResult;
use common::utils::error::AppError;

use crate::customer::json::customer::Customer;
use crate::customer::json::table::Customers;
use crate::pb::CreateCustomerRequest;

use sqlx::{Acquire, PgConnection, Postgres, Transaction};

#[derive(Debug, Default)]
pub struct PostgresCustomerRepo {}

impl PostgresCustomerRepo {
    pub async fn get<'c, C>(id: i64, conn: C) -> AppResult<Option<Customer>>
    where
        C: Acquire<'c, Database = Postgres>,
    {
        let mut conn = conn.acquire().await.unwrap();

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
