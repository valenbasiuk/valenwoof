pub mod commands;
pub mod models;


use sqlx::PgPool;

/// inicializa el pool de conexiones a la base de datos de postgres/supabase
pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}
