use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct DbApp {
   pub pool: PgPool
}

impl DbApp {
    pub async fn new() -> Result<Self> {
        use std::env;
        let db_url = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        Ok(Self {
            pool
        })
    }
}
