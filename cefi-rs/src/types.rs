use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Orderbook {
    pub symbol: String,
    pub asks: Vec<[String; 2]>,
    pub bids: Vec<[String; 2]>,
    pub timestamp_ms: i64,
}
