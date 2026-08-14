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

    let cfg = Config::from_env();

    println!("valenwoof arrancando...");

    bot::connect(&cfg).await;
}
