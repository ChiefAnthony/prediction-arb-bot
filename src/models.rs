use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct MarketUpdate {
    #[serde(rename = "marketId")]
    pub market_id: String,
    pub price: f64,
    pub outcome: String,
    pub timestamp: u64,
}

#[derive(Serialize, Debug, Clone)]
pub struct AuthPayload {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub secret: String,
    pub passphrase: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct SubscriptionMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthPayload>,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "assets_ids", skip_serializing_if = "Vec::is_empty")]
    pub assets_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub markets: Vec<String>,
}

type GenericMessage = serde_json::Value;
