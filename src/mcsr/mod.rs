pub mod client;
pub mod errors;
pub mod models;


// cliente y documentacion de la api de mcsrranked
//
// ### base url
// `https://api.mcsrranked.com`
//
// ### rate limit
// 500 requests por 10 minutos (sin auth)
// cache de 5 segundos en la mayoria de endpoints
//
// ---
//
// ## endpoints relevantes
//
// ### 1. GET /users/{identifier}
// perfil completo del jugador. identifier = nickname de minecraft o uuid.
//
// respuesta (data):
//   "uuid":          string,
//   "nickname":      string,
//   "roleType":      int,       // 0=normal, 1=mcsrpb, 2=staff, 3=special
//   "eloRate":       int|null,  // elo actual, null si no tiene ranked
//   "eloRank":       int|null,  // posicion en leaderboard
//   "country":       string|null,
//   "timestamp": {
//     "firstOnline": int,       // unix timestamp
//     "lastOnline":  int,
//     "lastRanked":  int,
//     "nextDecay":   int|null
//   },
//   "statistics": {
//     "season": { ... },        // stats de la temporada actual
//     "total":  { ... }         // stats historicos
//   }
//
// campos de statistics (identicos en season y total):
//   "bestTime":         { "ranked": int|null, "casual": int|null }  // milisegundos
//   "highestWinStreak": { "ranked": int, "casual": int }
//   "currentWinStreak": { "ranked": int, "casual": int }
//   "playedMatches":    { "ranked": int, "casual": int }
//   "playtime":         { "ranked": int, "casual": int }            // milisegundos
//   "completionTime":   { "ranked": int, "casual": int }
//   "completions":      { "ranked": int, "casual": int }
//   "forfeits":         { "ranked": int, "casual": int }
//   "wins":             { "ranked": int, "casual": int }
//   "loses":            { "ranked": int, "casual": int }
//
// ### 2. GET /users/{identifier}/matches
// historial de partidas. query params opcionales: count (default 10), before, after (match id)
//
// cada MatchInfo tiene:
//   "id":          int
//   "type":        int         // 3 = ranked
//   "category":    string      // "ANY"
//   "forfeited":   bool
//   "season":      int
//   "date":        int         // unix timestamp
//   "seedType":    string      // "VILLAGE", "SHIPWRECK", "RUINED_PORTAL", "BURIED_TREASURE", etc
//   "bastionType": string|null // "STABLES", "BRIDGE", "TREASURE", etc
//   "result": {
//     "uuid": string|null,     // uuid del ganador (null si ff)
//     "time": int|null         // tiempo en milisegundos
//   },
//   "players":  [ { "uuid", "nickname", "eloRate", "eloRank", ... } ]
//   "changes":  [ { "uuid": string, "change": int, "eloRate": int } ]
//
// ### 3. GET /matches/{match_id}
// detalle completo de una partida especifica
