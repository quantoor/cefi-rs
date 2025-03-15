use std::collections::HashMap;

use crate::http::BinanceHttp;
use crate::types::{ExchangeInfo, OrderBook, ServerTimeResponse};

pub async fn check_server_time(client: &BinanceHttp) -> anyhow::Result<ServerTimeResponse> {
    let server_time = client
        .send_get_request::<ServerTimeResponse>("fapi/v1/time", HashMap::new())
        .await?;
    Ok(server_time)
}

pub async fn get_exchange_info(client: &BinanceHttp) -> anyhow::Result<ExchangeInfo> {
    let response = client
        .send_get_request::<ExchangeInfo>("fapi/v1/exchangeInfo", HashMap::new())
        .await?;
    Ok(response)
}

pub async fn get_orderbook(
    client: &BinanceHttp,
    symbol: &String,
    _limit: Option<i32>,
) -> anyhow::Result<OrderBook> {
    let mut params = HashMap::new();
    params.insert("symbol", symbol.as_str());
    // if let Some(lim) = limit {
    //     let limit_str = lim.to_string().clone();
    //     params.insert("limit", limit_str.as_str());
    // }

    let response = client
        .send_get_request::<OrderBook>("fapi/v1/depth", params)
        .await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_server_time() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        let server_time = check_server_time(&binance_http).await.unwrap();
        println!("server_time: {:?}", server_time);
    }

    #[tokio::test]
    async fn test_get_exchange_info() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        let exchange_info = get_exchange_info(&binance_http).await.unwrap();
        println!("exchange_info: {:?}", exchange_info);
    }

    #[tokio::test]
    async fn test_get_orderbook() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        let orderbook = get_orderbook(&binance_http, &"BTCUSDT".to_string(), Some(10))
            .await
            .unwrap();
        println!("orderbook: {:?}", orderbook);
    }
}
