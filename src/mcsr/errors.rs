use std::fmt;

/// errores posibles al interactuar con la api de mcsrranked
#[derive(Debug)]
pub enum McsrError {
    UserNotFound(String),
    RateLimited,
    Http(reqwest::Error),
    ApiError(String),
}

impl fmt::Display for McsrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            McsrError::UserNotFound(user) => write!(f, "usuario '{user}' no encontrado"),
            McsrError::RateLimited => write!(f, "rate limit excedido en MCSRRanked API"),
            McsrError::Http(err) => write!(f, "error de conexion: {err}"),
            McsrError::ApiError(msg) => write!(f, "error de api: {msg}"),
        }
    }
}

impl std::error::Error for McsrError {}

impl From<reqwest::Error> for McsrError {
    fn from(err: reqwest::Error) -> Self {
        McsrError::Http(err)
    }
}
