use serde::{Deserialize, Serialize};

/// nivel de permiso para ejecutar un comando
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionLevel {
    Everyone,
    Mod,
    Broadcaster,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        Self::Everyone
    }
}

/// struct que representa una fila de la tabla custom_commands
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CustomCommand {

    pub id: i64,
    pub name: String,
    pub response: String,
    pub cooldown_seconds: i32,
    pub enabled: bool,
    pub permission_level: String,
}
