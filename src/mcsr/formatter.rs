use crate::mcsr::models::UserProfile;

/// formatea el tiempo en milisegundos a mm:ss (ej: 754000ms -> "12:34")
pub fn format_time_ms(ms: i64) -> String {
    let total_seconds = ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}:{seconds:02}")
}

/// formatea el perfil de un usuario para la respuesta del comando de chat
pub fn format_user_stats(profile: &UserProfile) -> String {
    let elo = profile
        .elo_rate
        .map_or("Unranked".to_string(), |e| e.to_string());

    let rank = profile
        .elo_rank
        .map_or("N/A".to_string(), |r| format!("#{r}"));

    let stats = &profile.statistics.total;

    let wins = stats.wins.ranked;
    let loses = stats.loses.ranked;
    let played = stats.played_matches.ranked;
    let forfeits = stats.forfeits.ranked;
    let completions = stats.completions.ranked;

    let winrate = if played > 0 {
        (wins as f64 / played as f64) * 100.0
    } else {
        0.0
    };

    let ff_rate = if played > 0 {
        (forfeits as f64 / played as f64) * 100.0
    } else {
        0.0
    };

    let pb_str = stats
        .best_time
        .ranked
        .map(format_time_ms)
        .unwrap_or_else(|| "N/A".to_string());

    let avg_str = if completions > 0 {
        let avg_ms = stats.completion_time.ranked / completions as i64;
        format_time_ms(avg_ms)
    } else {
        "N/A".to_string()
    };

    format!(
        "[{name}] ELO: {elo} (Rank {rank}) | W/L: {wins}W/{loses}L ({winrate:.1}%) | Matches: {played} | PB: {pb_str} | Avg: {avg_str} | FF Rate: {ff_rate:.1}%",
        name = profile.nickname,
    )
}
