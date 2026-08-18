use std::env;

/// configuracion del bot cargada desde variables de entorno
pub struct Config {
    pub bot_username: String,
    pub oauth_token: String,
    pub channel: String,
    pub twitch_client_id: String,
    pub twitch_client_secret: String,
    /// nombre de minecraft del streamer (usuario por default para !oshbt y !averages)
    pub mcsr_username: String,
}

impl Config {
    /// carga la config desde el entorno. panica si falta alguna variable obligatoria
    pub fn from_env() -> Self {
        Self {
            bot_username: require_env("TWITCH_BOT_USERNAME"),
            oauth_token: require_env("TWITCH_OAUTH_TOKEN"),
            channel: require_env("TWITCH_CHANNEL"),
            twitch_client_id: require_env("TWITCH_CLIENT_ID"),
            twitch_client_secret: require_env("TWITCH_CLIENT_SECRET"),
            mcsr_username: env::var("MCSR_USERNAME")
                .unwrap_or_else(|_| env::var("TWITCH_CHANNEL").unwrap_or_default()),
        }
    }
}

fn require_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("variable de entorno requerida no encontrada: {key}"))
}
