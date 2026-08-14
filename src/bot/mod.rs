use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use crate::config::Config;

pub mod commands;


/// arranca la conexion al chat de twitch, hace join al canal y loguea mensajes entrantes
pub async fn connect(cfg: &Config) {
    let login_config = ClientConfig::new_simple(StaticLoginCredentials::new(
        cfg.bot_username.clone(),
        Some(cfg.oauth_token.clone()),
    ));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(login_config);

    // arrancar tarea para consumir mensajes entrantes antes de hacer join
    let channel = cfg.channel.clone();
    let client_sender = client.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Join(msg) => {
                    println!("[join] {} entro a #{}", msg.user_login, msg.channel_login);
                }
                ServerMessage::Privmsg(msg) => {
                    println!("[chat] #{} | {}: {}", msg.channel_login, msg.sender.login, msg.message_text);

                    if let Some((cmd_name, _args)) = commands::parse_command(&msg.message_text) {
                        if cmd_name == "ping" {
                            let response = "pong!";
                            client_sender
                                .privmsg(msg.channel_login.clone(), response.to_string())
                                .await
                                .ok();
                            println!("[bot -> #{}] {}", msg.channel_login, response);
                        }
                    }
                }
                ServerMessage::Notice(msg) => {
                    println!("[notice] #{} | {}", msg.channel_login.as_deref().unwrap_or("?"), msg.message_text);
                }
                _ => {
                    // otros mensajes del servidor: ignorar por ahora
                }
            }
        }
        println!("[bot] canal #{}: stream de mensajes cerrado", channel);
    });


    // hacer join al canal
    client
        .join(cfg.channel.clone())
        .expect("fallo al hacer join al canal");

    println!("[bot] conectado a twitch irc, esperando join a #{}", cfg.channel);

    // mantener el loop corriendo hasta que el join_handle termine
    join_handle.await.unwrap();
}
