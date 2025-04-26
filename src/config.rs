use crate::error::{Error, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub websocket_url: String,
    // pub api_key: Option<String>, // Later
}

pub fn load_config() -> Result<Config> {
    dotenvy::dotenv().ok();

    let websocket_url = env::var("POLYMARKET_WEBSOCKET_URL")
        .map_err(|_| Error::Config("WEBSOCKET_URL must be set".into()))?;

    // let api_key = env::var("PREDICTION_MARKET_API_KEY").ok();

    Ok(Config {
        websocket_url,
        // api_key,
    })
}
