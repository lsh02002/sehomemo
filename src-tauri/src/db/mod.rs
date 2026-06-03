pub mod sqlite;

use sqlx::SqlitePool;
use sqlx::{sqlite::SqlitePoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // migrations 실행
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}