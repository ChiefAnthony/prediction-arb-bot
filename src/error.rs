use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("WebSocket connection error: {0}")]
    WebsocketConnection(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error), // keep for potential future REST calls yes okay

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("URL parsing error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
