// modulos principales del bot
mod bot;
mod config;
mod db;
mod mcsr;
mod spotify;

use config::Config;

#[tokio::main]
async fn main() {
    // cargar variables de entorno desde .env (si existe)
    dotenvy::dotenv().ok();

    // inicializar logger con tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cfg = Config::from_env();

    tracing::info!("valenwoof arrancando...");

    bot::connect(&cfg).await;
}

