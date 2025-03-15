use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookLevel {
    pub price: f64,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Orderbook {
    pub symbol: String,
    pub asks: Vec<OrderbookLevel>,
    pub bids: Vec<OrderbookLevel>,
    pub timestamp_ms: i64,
}
