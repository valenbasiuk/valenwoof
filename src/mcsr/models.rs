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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_profile() {
        let json_data = r#"{
            "status": "success",
            "data": {
                "uuid": "994f93763f8048bc9e72ee92f861911d",
                "nickname": "Couriway",
                "roleType": 3,
                "eloRate": 1500,
                "eloRank": 12,
                "statistics": {
                    "season": {
                        "bestTime": { "ranked": 555786, "casual": null },
                        "highestWinStreak": { "ranked": 8, "casual": 0 },
                        "currentWinStreak": { "ranked": 4, "casual": 0 },
                        "playedMatches": { "ranked": 10, "casual": 0 },
                        "playtime": { "ranked": 10000, "casual": 0 },
                        "forfeits": { "ranked": 1, "casual": 0 },
                        "completions": { "ranked": 5, "casual": 0 },
                        "wins": { "ranked": 7, "casual": 0 },
                        "loses": { "ranked": 3, "casual": 0 },
                        "completionTime": { "ranked": 2500000, "casual": 0 }
                    },
                    "total": {
                        "bestTime": { "ranked": 555786, "casual": null },
                        "highestWinStreak": { "ranked": 8, "casual": 2 },
                        "currentWinStreak": { "ranked": 4, "casual": 0 },
                        "playedMatches": { "ranked": 560, "casual": 6 },
                        "playtime": { "ranked": 432704086, "casual": 4991021 },
                        "forfeits": { "ranked": 62, "casual": 0 },
                        "completions": { "ranked": 212, "casual": 0 },
                        "wins": { "ranked": 305, "casual": 3 },
                        "loses": { "ranked": 244, "casual": 3 },
                        "completionTime": { "ranked": 192287574, "casual": 0 }
                    }
                }
            }
        }"#;

        let res: Result<ApiResponse<UserProfile>, _> = serde_json::from_str(json_data);
        assert!(res.is_ok(), "el json de perfil deberia ser valido");
        let profile = res.unwrap().data;
        assert_eq!(profile.nickname, "Couriway");
        assert_eq!(profile.elo_rate, Some(1500));
        assert_eq!(profile.elo_rank, Some(12));
        assert_eq!(profile.statistics.total.wins.ranked, 305);
    }

    #[test]
    fn test_parse_match_history() {
        let json_data = r#"{
            "status": "success",
            "data": [
                {
                    "id": 12367956,
                    "type": 3,
                    "category": "ANY",
                    "forfeited": false,
                    "decayed": false,
                    "season": 11,
                    "date": 1786570174,
                    "seedType": "VILLAGE",
                    "bastionType": "STABLES",
                    "result": {
                        "uuid": "994f93763f8048bc9e72ee92f861911d",
                        "time": 540000
                    },
                    "players": [
                        {
                            "uuid": "994f93763f8048bc9e72ee92f861911d",
                            "nickname": "Couriway",
                            "eloRate": 1500,
                            "eloRank": 12
                        }
                    ]
                }
            ]
        }"#;

        let res: Result<ApiResponse<Vec<MatchInfo>>, _> = serde_json::from_str(json_data);
        assert!(res.is_ok(), "el json de partidas deberia ser valido");
        let matches = res.unwrap().data;
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].seed_type, Some("VILLAGE".to_string()));
        assert_eq!(matches[0].result.time, Some(540000));
    }
}


