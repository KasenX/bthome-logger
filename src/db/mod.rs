pub mod queries;

use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
