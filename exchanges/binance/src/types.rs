use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    pub server_time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    // pub exchange_filters: Vec<serde_json::Value>, // empty array, using Value as catch-all
    pub rate_limits: Vec<RateLimit>,
    pub assets: Vec<Asset>,
    pub symbols: Vec<Symbol>,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub interval: String,
    pub interval_num: i32,
    pub limit: i32,
    pub rate_limit_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset: String,
    pub margin_available: bool,
    pub auto_asset_exchange: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: String,
    pub contract_type: String,
    pub delivery_date: i64,
    pub onboard_date: i64,
    pub status: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: String,
    pub price_precision: i32,
    pub quantity_precision: i32,
    pub base_asset_precision: i32,
    pub quote_precision: i32,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: Option<i32>,
    pub trigger_protect: String,
    // pub filters: Vec<Filter>,
    #[serde(rename = "OrderType")]
    pub order_types: Option<Vec<String>>,
    pub time_in_force: Vec<String>,
    pub liquidation_fee: String,
    pub market_take_bound: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        max_price: String,
        min_price: String,
        tick_size: String,
    },
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        max_qty: String,
        min_qty: String,
        step_size: String,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize {
        max_qty: String,
        min_qty: String,
        step_size: String,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders { limit: i32 },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders { limit: i32 },
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional { notional: String },
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        multiplier_decimal: i32,
    },
}

#[derive(Debug, Deserialize)]
pub struct OrderBook {
    pub last_update_id: Option<i64>,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    pub bids: Vec<[String; 2]>, // [price, quantity]
    pub asks: Vec<[String; 2]>, // [price, quantity]
}
