use crate::mcsr::models::UserProfile;
use crate::mcsr::models::MatchInfo;
use std::collections::HashMap;

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
        "[{name}] elo: {elo} | rank: {rank} | W/L: {wins}W/{loses}L ({winrate:.1}%) | matches: {played} | pb: {pb_str} | avg: {avg_str} | ff rate: {ff_rate:.1}%",
        name = profile.nickname,
    )
}

/// calcula y formatea el promedio de completions por categoria de seed
/// a partir del historial de partidas. solo usa partidas ranked completadas (no ff).
pub fn format_averages(nickname: &str, matches: &[MatchInfo]) -> String {
    // agrupar tiempos de completions por seed_type, solo ranked completadas
    let mut by_seed: HashMap<String, Vec<i64>> = HashMap::new();
    let mut overall_times: Vec<i64> = Vec::new();

    for m in matches {
        // solo ranked (type 3) y no forfeited con tiempo real
        if m.r#type != 3 || m.forfeited {
            continue;
        }
        if let Some(time_ms) = m.result.time {
            overall_times.push(time_ms);
            let seed = m
                .seed_type
                .clone()
                .unwrap_or_else(|| "UNKNOWN".to_string());
            by_seed.entry(seed).or_default().push(time_ms);
        }
    }

    if overall_times.is_empty() {
        return format!("[{nickname}] sin completions de ranked en las ultimas partidas");
    }

    let overall_avg = overall_times.iter().sum::<i64>() / overall_times.len() as i64;
    let overall_str = format_time_ms(overall_avg);

    // ordenar categorias por cantidad de completions (mayor primero)
    let mut breakdown: Vec<(String, i64, usize)> = by_seed
        .into_iter()
        .map(|(seed, times)| {
            let avg = times.iter().sum::<i64>() / times.len() as i64;
            (seed, avg, times.len())
        })
        .collect();
    breakdown.sort_by(|a, b| b.2.cmp(&a.2));

    let breakdown_str = breakdown
        .iter()
        .map(|(seed, avg_ms, count)| {
            let label = seed_label(seed);
            format!("{label}: {} ({})", format_time_ms(*avg_ms), count)
        })
        .collect::<Vec<_>>()
        .join(" | ");

    format!(
        "[{nickname}] Overall avg: {overall_str} ({n} completions) — {breakdown_str}",
        n = overall_times.len()
    )
}

/// convierte el nombre interno del seed a una abreviatura amigable para el chat
fn seed_label(seed: &str) -> &str {
    match seed {
        "VILLAGE" => "VIL",
        "SHIPWRECK" => "SHIP",
        "RUINED_PORTAL" => "RP",
        "BURIED_TREASURE" => "BT",
        "DESERT_TEMPLE" => "DT",
        "JUNGLE_TEMPLE" => "JT",
        _ => seed,
    }
}

