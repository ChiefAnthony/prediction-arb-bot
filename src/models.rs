use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct MarketUpdate {
    #[serde(rename = "marketId")]
    pub market_id: String,
    pub price: f64,
    pub outcome: String,
    pub timestamp: u64,
}
