// modulos principales del bot
mod bot;
mod config;
mod db;
mod mcsr;
mod spotify;


#[tokio::main]
async fn main() {
    // cargar variables de entorno desde .env (si existe)
    dotenvy::dotenv().ok();

    println!("valenwoof arrancando...");
}

