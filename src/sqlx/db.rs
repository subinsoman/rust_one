use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::config::application_config::AppConfig;

static POOL: Lazy<Mutex<Option<MySqlPool>>> = Lazy::new(|| Mutex::new(None));

pub async fn establish_connection(config:AppConfig) -> Result<(), sqlx::Error> {
    let mut pool = POOL.lock().unwrap();

    if pool.is_none() {
        let new_pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&config.database.url)
            .await?;

        *pool = Some(new_pool);
    }

    Ok(())
}

pub fn get_pool() -> MySqlPool {
    let pool = POOL.lock().unwrap();
    pool.as_ref().expect("Database pool not initialized").clone()
}
