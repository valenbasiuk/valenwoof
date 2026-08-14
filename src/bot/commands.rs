use twitch_irc::message::PrivmsgMessage;

/// un comando del bot: nombre sin el `!` y funcion asincrona que lo maneja
pub struct Command {
    pub name: &'static str,
    pub handler: fn(&PrivmsgMessage) -> String,
}

/// lista de comandos hardcodeados del bot
pub fn builtin_commands() -> Vec<Command> {
    vec![Command {
        name: "ping",
        handler: |_msg| "pong!".to_string(),
    }]
}

/// intenta parsear un mensaje de chat como un comando.
/// retorna (nombre, argumentos) si empieza con `!`, None si no.
pub fn parse_command(text: &str) -> Option<(&str, &str)> {
    let text = text.trim();
    if !text.starts_with('!') {
        return None;
    }
    let without_prefix = &text[1..];
    let (name, args) = without_prefix
        .split_once(' ')
        .unwrap_or((without_prefix, ""));
    Some((name, args))
}
