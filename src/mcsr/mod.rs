//! cliente y documentacion de la api de mcsrranked (api.mcsrranked.com)
//!
//! ### base url
//! `https://api.mcsrranked.com`
//!
//! ### rate limit
//! 500 requests por 10 minutos (publico, sin auth).
//!
//! ### endpoints principales:
//! 1. `GET /users/{identifier}`
//!    - trae perfil del jugador (elo, rank, win/loss, best time, etc.)
//!    - identifier: nickname o UUID de Minecraft
//!
//! 2. `GET /users/{identifier}/matches`
//!    - trae el historial de partidas del jugador (para promedios, ff rate, etc.)
//!    - soporta query params: `before`, `after`, `type`
//!
//! 3. `GET /matches/{match_id}`
//!    - trae el detalle completo de una partida especifica
