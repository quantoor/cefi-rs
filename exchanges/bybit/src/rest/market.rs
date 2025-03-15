use std::collections::HashMap;

use crate::{errors::BybitResult, http::BybitHttp, types::GetTickersResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    pub time_second: String,
    pub time_nano: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoResponse {
    pub category: String,
    pub list: Vec<InstrumentInfo>,
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    pub symbol: String,
    pub contract_type: String,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_scale: String,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i32,
    pub settle_coin: String,
    pub copy_trading: String,
    pub upper_funding_rate: String,
    pub lower_funding_rate: String,
    pub is_pre_listing: bool,
    pub pre_listing_info: Option<serde_json::Value>,
    pub risk_parameters: RiskParameters,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    pub min_leverage: String,
    pub max_leverage: String,
    pub leverage_step: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    pub max_order_qty: String,
    pub min_order_qty: String,
    pub qty_step: String,
    pub post_only_max_order_qty: String,
    pub max_mkt_order_qty: String,
    pub min_notional_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    pub price_limit_ratio_x: String,
    pub price_limit_ratio_y: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookResponse {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub asks: Vec<[String; 2]>,
    #[serde(rename = "b")]
    pub bids: Vec<[String; 2]>,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "u")]
    pub update_id: i64,
    pub seq: i64,
    #[serde(rename = "cts")]
    pub cross_seq: i64,
}

impl BybitHttp {
    pub async fn get_server_time(&self) -> BybitResult<ServerTimeResponse> {
        self.send_get_request::<ServerTimeResponse>("v5/market/time", HashMap::new(), false)
            .await
    }

    pub async fn get_instruments_info(
        &self,
        category: String,
    ) -> BybitResult<InstrumentsInfoResponse> {
        self.send_get_request::<InstrumentsInfoResponse>(
            "v5/market/instruments-info",
            HashMap::from([("category", category.as_str())]),
            false,
        )
        .await
    }

    pub async fn get_orderbook(
        &self,
        category: String,
        symbol: String,
        limit: i32,
    ) -> BybitResult<OrderbookResponse> {
        self.send_get_request::<OrderbookResponse>(
            "v5/market/orderbook",
            HashMap::from([
                ("category", category.as_str()),
                ("symbol", symbol.as_str()),
                ("limit", limit.to_string().as_str()),
            ]),
            false,
        )
        .await
    }

    pub async fn get_tickers(&self) -> BybitResult<GetTickersResponse> {
        self.send_get_request::<GetTickersResponse>(
            "v5/market/tickers",
            HashMap::from([("category", "linear")]),
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_server_time() {
        let client = BybitHttp::new("".to_string(), "".to_string());
        let res = client.get_server_time().await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_instruments_info() {
        let client = BybitHttp::new("".to_string(), "".to_string());
        let res = client.get_instruments_info("linear".to_string()).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_orderbook() {
        let client = BybitHttp::new("".to_string(), "".to_string());
        let res = client
            .get_orderbook("linear".to_string(), "BTCUSDT".to_string(), 5)
            .await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_tickers() {
        let client = BybitHttp::new("".to_string(), "".to_string());
        let res = client.get_tickers().await;
        println!("{:?}", res);
    }
}
