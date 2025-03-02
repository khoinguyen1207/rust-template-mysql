
use crate::{configs::configs, serializes::error::AppError, };
use once_cell::sync::Lazy;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::sync::Arc;

pub static DB_POOL: Lazy<Arc<MySqlPool>> = Lazy::new(|| {
    let database_url: String = configs::get("database_url");
    let pool = MySqlPoolOptions::new()
        .max_connections(200)
        .connect_lazy(&database_url)
        .expect("Failed to create pool");
    Arc::new(pool)
});

pub fn get_db_pool() -> Arc<MySqlPool> {
    DB_POOL.clone()
}

pub async fn migrate_db() -> Result<(), AppError> {
    let pool = get_db_pool();

    sqlx::migrate!()
        .run(&*pool)
        .await
        .map_err(|e| AppError::new(500).message(&e.to_string()))?;
    Ok(())
}



