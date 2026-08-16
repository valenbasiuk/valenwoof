use reqwest::{Client, StatusCode};

use crate::mcsr::errors::McsrError;
use crate::mcsr::models::{ApiResponse, MatchInfo, UserProfile};

const BASE_URL: &str = "https://api.mcsrranked.com";

/// cliente http para la api de mcsrranked
pub struct McsrClient {
    http: Client,
}

impl McsrClient {
    /// crea un nuevo cliente con un user-agent identificable
    pub fn new() -> Self {
        let http = Client::builder()
            .user_agent("valenwoof-bot/0.1")
            .build()
            .expect("fallo al construir el http client");
        Self { http }
    }

    /// construye la url completa para un path dado
    pub fn url(&self, path: &str) -> String {
        format!("{BASE_URL}{path}")
    }

    /// referencia al cliente http interno (para hacer requests en otros modulos)
    pub fn http(&self) -> &Client {
        &self.http
    }

    /// obtiene el perfil completo de un usuario por su nickname o uuid
    pub async fn get_user_profile(
        &self,
        identifier: &str,
    ) -> Result<UserProfile, McsrError> {
        let url = self.url(&format!("/users/{identifier}"));
        let res = self.http.get(&url).send().await?;

        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(McsrError::RateLimited);
        }
        if res.status() == StatusCode::NOT_FOUND {
            return Err(McsrError::UserNotFound(identifier.to_string()));
        }

        let api_res = res.json::<ApiResponse<UserProfile>>().await;
        match api_res {
            Ok(parsed) => {
                if parsed.status == "error" {
                    Err(McsrError::UserNotFound(identifier.to_string()))
                } else {
                    Ok(parsed.data)
                }
            }
            Err(_) => Err(McsrError::UserNotFound(identifier.to_string())),
        }
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
