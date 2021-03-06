use async_trait::async_trait;
use futures::FutureExt;
use sea_query::{Cond, Query};
use sea_query::{Expr, PostgresQueryBuilder};

use common::utils::alias::{AppResult, PostgresAcquire};
use common::utils::error::AppError;

use crate::customer::json::customer::Customer;
use crate::customer::json::table::Customers;
use crate::customer::repo::CustomerRepo;
use crate::pb::{CreateCustomerRequest, ListCustomerRequest, UpdateCustomerRequest};
use crate::ID_GENERATOR;

pub struct CustomerRepoImpl;

/// references: https://qiita.com/FuJino/items/08b4c3298918191eab65

#[async_trait]
impl CustomerRepo for CustomerRepoImpl {
    async fn get(
        &self,
        id: i64,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Option<Customer>> {
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

        sqlx::query_as::<_, Customer>(&sql)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn create(
        &self,
        request: CreateCustomerRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Customer> {
        let mut conn = executor.acquire().await.unwrap();

        use chrono::Utc;

        let id = async move { ID_GENERATOR.clone().lock().unwrap().next_id() }
            .boxed()
            .await as u64;

        let name = request.name.clone().into();
        let email = request.email.into();
        let phone = request.phone.into();
        let created_at = Utc::now().into();

        let cols: Vec<Customers> = vec![
            Customers::Id,
            Customers::Name,
            Customers::Email,
            Customers::Phone,
            Customers::CreatedAt,
            Customers::UpdatedAt,
        ];

        let sql = Query::insert()
            .into_table(Customers::Table)
            .columns(cols.clone().into_iter().take(5).collect::<Vec<_>>())
            .values_panic(vec![id.into(), name, email, phone, created_at])
            .returning(Query::select().columns(cols).take())
            .to_string(PostgresQueryBuilder);

        sqlx::query_as::<_, Customer>(&sql)
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn list(
        &self,
        request: ListCustomerRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<Vec<Customer>> {
        let mut conn = executor.acquire().await.unwrap();

        let cursor = request.cursor;
        let query = request.query.map(|q| format!("%{}%", q));
        let page_size = request.page_size;

        let sql = Query::select()
            .columns(vec![
                Customers::Id,
                Customers::Name,
                Customers::Email,
                Customers::Phone,
                Customers::CreatedAt,
                Customers::UpdatedAt,
            ])
            .cond_where(Cond::all().add_option(cursor.map(|e| Expr::col(Customers::Id).eq(e))))
            .cond_where(
                Cond::any()
                    .add_option(query.clone().map(|e| Expr::col(Customers::Name).like(&e)))
                    .add_option(query.clone().map(|e| Expr::col(Customers::Email).like(&e)))
                    .add_option(query.map(|e| Expr::col(Customers::Phone).like(&e))),
            )
            .from(Customers::Table)
            .limit(page_size as u64)
            .to_string(PostgresQueryBuilder);

        sqlx::query_as::<_, Customer>(&sql)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn update(
        &self,
        request: UpdateCustomerRequest,
        executor: impl PostgresAcquire<'_> + 'async_trait,
    ) -> AppResult<bool> {
        let mut conn = executor.acquire().await.unwrap();

        let mut update_values = vec![];

        if let Some(name) = request.name {
            update_values.push((Customers::Name, name.into()));
        }

        if let Some(email) = request.email {
            update_values.push((Customers::Email, email.into()));
        }

        if let Some(phone) = request.phone {
            update_values.push((Customers::Phone, phone.into()));
        }

        let sql = Query::update()
            .table(Customers::Table)
            .values(update_values)
            .and_where(Expr::col(Customers::Id).eq(request.id))
            .to_string(PostgresQueryBuilder);

        sqlx::query(&sql)
            .execute(&mut *conn)
            .await
            .map(|e| e.rows_affected() > 0)
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
