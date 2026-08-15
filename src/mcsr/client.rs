use reqwest::Client;

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
}
