use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use crate::config::Config;

/// arranca la conexion al chat de twitch y loguea que se conecto
pub async fn connect(cfg: &Config) {
    let login_config = ClientConfig::new_simple(StaticLoginCredentials::new(
        cfg.bot_username.clone(),
        Some(cfg.oauth_token.clone()),
    ));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(login_config);

    // arrancar tarea para consumir mensajes entrantes (necesario para que el cliente funcione)
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("mensaje recibido: {:?}", message);
        }
    });

    // unirse al canal
    client.join(cfg.channel.clone()).expect("fallo al hacer join al canal");

    println!("conectado a twitch irc, canal: #{}", cfg.channel);

    // esperar a que el loop de mensajes termine (corre para siempre en condiciones normales)
    join_handle.await.unwrap();
}
