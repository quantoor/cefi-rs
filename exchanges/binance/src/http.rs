use crate::{errors::BinanceResult, types::ServerTimeResponse};
use hmac::Hmac;
use reqwest::{Client, RequestBuilder};
use serde_json::{Map, Value};
use sha2::Sha256;
use std::collections::HashMap;

static BINANCE_HOST_HOST: &'static str = "https://fapi.binance.com";

pub struct BinanceHttp {
    api_key: String,
    api_secret: String,
    client: Client,
}

impl BinanceHttp {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("reqwest client"),
        }
    }

    async fn send_get_request<T>(&self, endpoint: &str) -> anyhow::Result<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        let response = self
            .client
            .get(format!("{}/{}", BINANCE_HOST_HOST, endpoint))
            .send()
            .await?;
        let body = response.text().await?;
        Ok(serde_json::from_str::<T>(&body)?)
    }

    pub async fn check_server_time(&self) -> anyhow::Result<ServerTimeResponse> {
        let server_time = self
            .send_get_request::<ServerTimeResponse>("fapi/v1/time")
            .await?;
        Ok(server_time)
    }

    pub async fn get_exchange_info(&self) -> anyhow::Result<()> {
        let response = self
            .client
            .get(format!("{}/fapi/v1/exchangeInfo", BINANCE_HOST_HOST))
            .send()
            .await?;
        let body = response.text().await?;
        println!("body: {:?}", body);
        Ok(())
    }

    pub async fn get_orderbook(&self) -> anyhow::Result<()> {
        let response = self
            .client
            .get(format!(
                "{}/fapi/v1/depth?symbol=BTCUSDT&limit=5",
                BINANCE_HOST_HOST
            ))
            .send()
            .await?;
        let body = response.text().await?;
        println!("body: {:?}", body);
        Ok(())
    }

    // fn generate_get_signature(
    //     &self,
    //     timestamp: &str,
    //     params: &HashMap<&str, &str>,
    // ) -> BinanceResult<String> {
    //     todo!()
    // }

    // fn generate_post_signature(
    //     &self,
    //     timestamp: &str,
    //     params: &Map<String, Value>,
    // ) -> BinanceResult<String> {
    //     todo!()
    // }

    // fn generate_query_str(params: &HashMap<&str, &str>) -> String {
    //     params
    //         .iter()
    //         .map(|(key, value)| format!("{}={}", key, value))
    //         .collect::<Vec<String>>()
    //         .join("&")
    // }

    // fn add_headers(
    //     &self,
    //     builder: RequestBuilder,
    //     timestamp: &String,
    //     signature: &String,
    // ) -> RequestBuilder {
    //     builder
    //         .header("X-BAPI-API-KEY", self.api_key.clone())
    //         .header("X-BAPI-SIGN", signature)
    //         .header("X-BAPI-SIGN-TYPE", "2")
    //         .header("X-BAPI-TIMESTAMP", timestamp.clone())
    //         .header("X-BAPI-RECV-WINDOW", RECV_WINDOW)
    //         .header("Content-Type", "application/json")
    // }

    // async fn send_get_request<T>(
    //     &self,
    //     endpoint: &str,
    //     params: HashMap<&str, &str>,
    //     is_auth: bool,
    // ) -> BinanceResult<T>
    // where
    //     T: for<'a> serde::Deserialize<'a>,
    // {
    //     todo!()
    // }

    // async fn send_post_request<T>(
    //     &self,
    //     endpoint: &str,
    //     params: Map<String, Value>,
    // ) -> BinanceResult<T>
    // where
    //     T: for<'a> serde::Deserialize<'a>,
    // {
    //     todo!()
    // }

    // pub async fn get_open_order(
    //     &self,
    //     symbol: &String,
    //     cloid: Option<String>,
    // ) -> BinanceResult<GetOrderResponse> {
    //     todo!()
    // }

    // pub async fn place_order(
    //     &self,
    //     cloid: &String,
    //     symbol: &String,
    //     side: &String,
    //     price: &String,
    //     qty: &String,
    // ) -> BinanceResult<OrderResponse> {
    //     todo!()
    // }

    // pub async fn cancel_order(
    //     &self,
    //     symbol: &String,
    //     cloid: &String,
    // ) -> BinanceResult<OrderResponse> {
    //     todo!()
    // }

    // pub async fn cancel_all_orders(
    //     &self,
    //     symbol: &String,
    // ) -> BinanceResult<CancelAllOrdersResponse> {
    //     todo!()
    // }

    // pub async fn amend_order(
    //     &self,
    //     symbol: &String,
    //     cloid: &String,
    //     price: &String,
    //     // qty: &String,
    // ) -> BinanceResult<OrderResponse> {
    //     todo!()
    // }

    // pub async fn get_orderbook(&self, symbol: &String) -> BinanceResult<OrderbookResponse> {
    //     todo!()
    // }

    // pub async fn get_positions(&self) -> BinanceResult<GetPositionResponse> {
    //     todo!()
    // }

    // pub async fn get_tickers(&self) -> BinanceResult<GetTickersResponse> {
    //     todo!()
    // }

    // pub async fn get_wallet_balance(&self) -> BinanceResult<GetWalletBalanceResponse> {
    //     todo!()
    // }

    // pub async fn get_account_info(&self) -> BinanceResult<BybitAccountInfo> {
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_server_time() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        let server_time = binance_http.check_server_time().await.unwrap();
        println!("server_time: {:?}", server_time);
    }

    #[tokio::test]
    async fn exchange_info() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        binance_http.get_exchange_info().await.unwrap();
    }

    #[tokio::test]
    async fn get_orderbook() {
        let binance_http = BinanceHttp::new("api_key".to_string(), "api_secret".to_string());
        binance_http.get_orderbook().await.unwrap();
    }
}
