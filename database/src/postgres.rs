use async_trait::async_trait;
use models::user::User;
use models::repository::UserRepository;
use sql_builder::prelude::*;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::env;
use std::error::Error;
use std::result::Result;

use crate::db_error::DatabaseError;

#[derive(Debug)]
pub struct Postgres {
    pool: sqlx::PgPool,
}

impl Postgres {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");

        let pool = PgPool::new(&database_url)
            .await
            .expect("could not create postgres connnection pool");

        Postgres { pool }
    }
}

#[async_trait]
impl UserRepository for Postgres {
    async fn create(&self, user: &User) -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::insert_into("users")
            .field("name")
            .field("password")
            .values(&["$1, $2"])
            .sql()
            .map_err(DatabaseError::from)?;

        let mut tx = self.pool.begin().await?;

        sqlx::query(&sql)
            .bind(&user.name)
            .bind(&user.password)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn find_by_id(&self, id: String) -> Result<User, Box<dyn Error>> {
        let sql = SqlBuilder::select_from("users")
            .fields(&["id", "name", "password"])
            .and_where("id = ?".bind(&id))
            .sql()
            .map_err(DatabaseError::from)?;

        let user = sqlx::query_as::<_, User>(&sql).fetch_one(&self.pool).await?;

        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let sql = SqlBuilder::select_from("users")
            .fields(&["id", "name", "password"])
            .order_by("id", false)
            .sql()
            .map_err(DatabaseError::from)?;

        let users = sqlx::query_as::<_, User>(&sql).fetch_all(&self.pool).await?;

        Ok(users)
    }
}
