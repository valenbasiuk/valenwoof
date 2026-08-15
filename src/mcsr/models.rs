use serde::{Deserialize, Serialize};

/// respuesta wrapper de la api de mcsrranked: {"status": "success", "data": ...}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

/// perfil de usuario retornado por GET /users/{identifier}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub uuid: String,
    pub nickname: String,
    pub role_type: Option<i32>,
    pub elo_rate: Option<i32>,
    pub elo_rank: Option<i32>,
    pub statistics: StatisticsContainer,
}

/// contenedor de estadísticas por temporada y totales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsContainer {
    pub season: Statistics,
    pub total: Statistics,
}

/// estadisticas de partidas (ranked / casual)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub best_time: ModeStat<Option<i64>>,
    pub highest_win_streak: ModeStat<i32>,
    pub current_win_streak: ModeStat<i32>,
    pub played_matches: ModeStat<i32>,
    pub playtime: ModeStat<i64>,
    pub forfeits: ModeStat<i32>,
    pub completions: ModeStat<i32>,
    pub wins: ModeStat<i32>,
    pub loses: ModeStat<i32>,
    pub completion_time: ModeStat<i64>,
}

/// desglose de valor entre ranked y casual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeStat<T> {
    pub ranked: T,
    pub casual: T,
}

/// partida retornado por GET /users/{identifier}/matches
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub id: i64,
    pub r#type: i32,
    pub category: Option<String>,
    pub forfeited: bool,
    pub decayed: Option<bool>,
    pub season: Option<i32>,
    pub date: i64,
    pub seed_type: Option<String>,
    pub bastion_type: Option<String>,
    pub result: MatchResult,
    pub players: Vec<MatchPlayer>,
}

/// resultado de una partida
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub uuid: Option<String>,
    pub time: Option<i64>,
}

/// jugador dentro de una partida
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchPlayer {
    pub uuid: String,
    pub nickname: String,
    pub elo_rate: Option<i32>,
    pub elo_rank: Option<i32>,
}

