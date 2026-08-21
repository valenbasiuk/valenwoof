use sqlx::PgPool;

use crate::db::models::CustomCommand;

/// obtiene todos los comandos custom habilitados desde la base de datos
pub async fn get_all_enabled_commands(pool: &PgPool) -> Result<Vec<CustomCommand>, sqlx::Error> {
    sqlx::query_as::<_, CustomCommand>(
        "SELECT id, name, response, cooldown_seconds, enabled, permission_level FROM custom_commands WHERE enabled = true ORDER BY name ASC"
    )
    .fetch_all(pool)
    .await
}

/// obtiene un comando custom especifico por su nombre (si esta habilitado)
pub async fn get_command_by_name(
    pool: &PgPool,
    name: &str,
) -> Result<Option<CustomCommand>, sqlx::Error> {
    sqlx::query_as::<_, CustomCommand>(
        "SELECT id, name, response, cooldown_seconds, enabled, permission_level FROM custom_commands WHERE name = $1 AND enabled = true"
    )
    .bind(name)
    .fetch_optional(pool)
    .await
}
