use std::collections::HashMap;

use crate::http::BinanceHttp;
use crate::types::{ExchangeInfo, OrderBook, ServerTimeResponse};

impl BinanceHttp {
    pub async fn check_server_time(&self) -> anyhow::Result<ServerTimeResponse> {
        let server_time = self
            .send_get_request::<ServerTimeResponse>("fapi/v1/time", HashMap::new())
            .await?;
        Ok(server_time)
    }

    pub async fn get_exchange_info(&self) -> anyhow::Result<ExchangeInfo> {
        let response = self
            .send_get_request::<ExchangeInfo>("fapi/v1/exchangeInfo", HashMap::new())
            .await?;
        Ok(response)
    }

    pub async fn get_orderbook(
        &self,
        symbol: &String,
        limit: Option<i32>,
    ) -> anyhow::Result<OrderBook> {
        let limit = limit.unwrap_or(5).to_string();

        let mut params = HashMap::new();
        params.insert("symbol", symbol.as_str());
        params.insert("limit", limit.as_str());

        let response = self
            .send_get_request::<OrderBook>("fapi/v1/depth", params)
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_server_time() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        let server_time = binance_http.check_server_time().await.unwrap();
        println!("server_time: {:?}", server_time);
    }

    #[tokio::test]
    async fn test_get_exchange_info() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        let exchange_info = binance_http.get_exchange_info().await.unwrap();
        println!("exchange_info: {:?}", exchange_info);
    }

    #[tokio::test]
    async fn test_get_orderbook() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        let orderbook = binance_http
            .get_orderbook(&"BTCUSDT".to_string(), Some(10))
            .await
            .unwrap();
        println!("orderbook: {:?}", orderbook);
    }
}
