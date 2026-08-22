use std::sync::Arc;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use crate::config::Config;
use crate::db;
use crate::mcsr::client::McsrClient;
use crate::mcsr::formatter;

pub mod commands;

/// arranca la conexion al chat de twitch, hace join al canal y loguea mensajes entrantes
pub async fn connect(cfg: &Config) {
    let login_config = ClientConfig::new_simple(StaticLoginCredentials::new(
        cfg.bot_username.clone(),
        Some(cfg.oauth_token.clone()),
    ));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(login_config);

    let channel = cfg.channel.clone();
    let client_sender = client.clone();
    let default_mcsr_user = cfg.mcsr_username.clone();
    let mcsr = Arc::new(McsrClient::new());

    // inicializar db pool si hay database_url configurada
    let db_pool = if let Some(ref db_url) = cfg.database_url {
        match db::init_pool(db_url).await {
            Ok(pool) => {
                tracing::info!("[db] conexion exitosa a la base de datos");
                Some(pool)
            }
            Err(e) => {
                tracing::error!("[db] error al conectar a la base de datos: {e}");
                None
            }
        }
    } else {
        tracing::warn!("[db] DATABASE_URL no configurada, comandos custom deshabilitados");
        None
    };

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Join(msg) => {
                    tracing::info!("[join] {} entro a #{}", msg.user_login, msg.channel_login);
                }
                ServerMessage::Privmsg(msg) => {
                    tracing::info!("[chat] #{} | {}: {}", msg.channel_login, msg.sender.login, msg.message_text);

                    if let Some((cmd_name, args)) = commands::parse_command(&msg.message_text) {
                        let channel_clone = msg.channel_login.clone();
                        let sender_clone = client_sender.clone();
                        let mcsr_clone = Arc::clone(&mcsr);
                        let default_user = default_mcsr_user.clone();

                        // 1. buscar primero en comandos custom de la base de datos
                        let mut custom_response = None;
                        if let Some(ref pool) = db_pool {
                            if let Ok(Some(cmd)) = db::commands::get_command_by_name(pool, cmd_name).await {
                                custom_response = Some(cmd.response);
                            }
                        }

                        // 2. si habia comando custom, usar su respuesta; si no, buscar en hardcodeados
                        let response = if let Some(resp) = custom_response {
                            Some(resp)
                        } else {
                            match cmd_name {
                                "ping" => Some("pong!".to_string()),

                                "oshbt" => {
                                    let user = if args.is_empty() { &default_user } else { args };
                                    match mcsr_clone.get_user_profile(user).await {
                                        Ok(profile) => Some(formatter::format_user_stats(&profile)),
                                        Err(e) => Some(format!("error: {e}")),
                                    }
                                }

                                "averages" => {
                                    let user = if args.is_empty() { &default_user } else { args };
                                    match mcsr_clone.get_user_matches(user, Some(50)).await {
                                        Ok(matches) => Some(formatter::format_averages(user, &matches)),
                                        Err(e) => Some(format!("error: {e}")),
                                    }
                                }

                                _ => None,
                            }
                        };

                        if let Some(text) = response {
                            sender_clone.privmsg(channel_clone.clone(), text.clone()).await.ok();
                            tracing::info!("[bot -> #{}] {}", channel_clone, text);
                        }
                    }
                }
                ServerMessage::Notice(msg) => {
                    tracing::info!("[notice] #{} | {}", msg.channel_login.as_deref().unwrap_or("?"), msg.message_text);
                }
                ServerMessage::Reconnect(_) => {
                    tracing::warn!("[bot] twitch pidio reconexion, reconectando automaticamente...");
                }
                _ => {}
            }
        }
        tracing::warn!("[bot] canal #{}: stream de mensajes cerrado", channel);
    });

    client
        .join(cfg.channel.clone())
        .expect("fallo al hacer join al canal");

    tracing::info!("[bot] conectado a twitch irc, esperando join a #{}", cfg.channel);

    join_handle.await.unwrap();
}
