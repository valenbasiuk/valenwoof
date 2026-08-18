use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::{Client, StatusCode};
use tokio::sync::RwLock;

use crate::mcsr::errors::McsrError;
use crate::mcsr::models::{ApiResponse, MatchInfo, UserProfile};

const BASE_URL: &str = "https://api.mcsrranked.com";
const CACHE_TTL_SECONDS: u64 = 60;

/// cliente http para la api de mcsrranked con cache en memoria (TTL 60s)
pub struct McsrClient {
    http: Client,
    profile_cache: Arc<RwLock<HashMap<String, (UserProfile, Instant)>>>,
    ttl: Duration,
}

impl McsrClient {
    /// crea un nuevo cliente con un user-agent identificable y cache activado
    pub fn new() -> Self {
        let http = Client::builder()
            .user_agent("valenwoof-bot/0.1")
            .build()
            .expect("fallo al construir el http client");
        Self {
            http,
            profile_cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(CACHE_TTL_SECONDS),
        }
    }

    /// construye la url completa para un path dado
    pub fn url(&self, path: &str) -> String {
        format!("{BASE_URL}{path}")
    }

    /// referencia al cliente http interno
    pub fn http(&self) -> &Client {
        &self.http
    }

    /// obtiene el perfil completo de un usuario por su nickname o uuid (usando cache)
    pub async fn get_user_profile(
        &self,
        identifier: &str,
    ) -> Result<UserProfile, McsrError> {
        let key = identifier.to_lowercase();

        // 1. intentar leer del cache
        {
            let cache = self.profile_cache.read().await;
            if let Some((profile, timestamp)) = cache.get(&key) {
                if timestamp.elapsed() < self.ttl {
                    return Ok(profile.clone());
                }
            }
        }

        // 2. si no esta en cache o expiro, consultar a la api
        let url = self.url(&format!("/users/{identifier}"));
        let res = self.http.get(&url).send().await?;

        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(McsrError::RateLimited);
        }
        if res.status() == StatusCode::NOT_FOUND {
            return Err(McsrError::UserNotFound(identifier.to_string()));
        }

        let api_res = res.json::<ApiResponse<UserProfile>>().await;
        let profile = match api_res {
            Ok(parsed) => {
                if parsed.status == "error" {
                    return Err(McsrError::UserNotFound(identifier.to_string()));
                } else {
                    parsed.data
                }
            }
            Err(_) => return Err(McsrError::UserNotFound(identifier.to_string())),
        };

        // 3. guardar en cache
        {
            let mut cache = self.profile_cache.write().await;
            cache.insert(key, (profile.clone(), Instant::now()));
        }

        Ok(profile)
    }

    /// obtiene el historial de partidas recientes de un usuario
    pub async fn get_user_matches(
        &self,
        identifier: &str,
        count: Option<u32>,
    ) -> Result<Vec<MatchInfo>, McsrError> {
        let count_val = count.unwrap_or(20);
        let url = self.url(&format!("/users/{identifier}/matches?count={count_val}"));
        let res = self.http.get(&url).send().await?;

        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(McsrError::RateLimited);
        }

        let api_res = res.json::<ApiResponse<Vec<MatchInfo>>>().await?;
        Ok(api_res.data)
    }
}
