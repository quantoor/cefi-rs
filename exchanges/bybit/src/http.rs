use crate::{
    errors::{BybitError, BybitResult},
    types::*,
};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{Client, RequestBuilder};
use serde_json::{Map, Value, json};
use sha2::Sha256;
use std::collections::HashMap;
use tracing::debug;

static BYBIT_HOST: &'static str = "https://api.bybit.com";
static RECV_WINDOW: &'static str = "5000";

pub struct BybitHttp {
    api_key: String,
    api_secret: String,
    client: Client,
}

impl BybitHttp {
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

    fn generate_get_signature(
        &self,
        timestamp: &str,
        params: &HashMap<&str, &str>,
    ) -> BybitResult<String> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(timestamp.as_bytes());
        mac.update(self.api_key.as_bytes());
        mac.update(RECV_WINDOW.as_bytes());
        mac.update(Self::generate_query_str(params).as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        Ok(hex::encode(code_bytes))
    }

    fn generate_post_signature(
        &self,
        timestamp: &str,
        params: &Map<String, Value>,
    ) -> BybitResult<String> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(timestamp.as_bytes());
        mac.update(self.api_key.as_bytes());
        mac.update(RECV_WINDOW.as_bytes());
        mac.update(serde_json::to_string(&params)?.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        Ok(hex::encode(code_bytes))
    }

    fn generate_query_str(params: &HashMap<&str, &str>) -> String {
        params
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&")
    }

    fn add_headers(
        &self,
        builder: RequestBuilder,
        timestamp: &String,
        signature: &String,
    ) -> RequestBuilder {
        builder
            .header("X-BAPI-API-KEY", self.api_key.clone())
            .header("X-BAPI-SIGN", signature)
            .header("X-BAPI-SIGN-TYPE", "2")
            .header("X-BAPI-TIMESTAMP", timestamp.clone())
            .header("X-BAPI-RECV-WINDOW", RECV_WINDOW)
            .header("Content-Type", "application/json")
    }

    async fn send_get_request<T>(
        &self,
        endpoint: &str,
        params: HashMap<&str, &str>,
        is_auth: bool,
    ) -> BybitResult<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        let query_str = Self::generate_query_str(&params);

        let mut builder = self
            .client
            .get(&format!("{}/{}?{}", BYBIT_HOST, endpoint, query_str));

        if is_auth {
            let timestamp = Utc::now().timestamp_millis().to_string();
            let signature = self.generate_get_signature(&timestamp, &params)?;
            builder = self.add_headers(builder, &timestamp, &signature)
        }

        let response = builder
            .send()
            .await
            .map_err(|err| BybitError::Unknown(format!("{}", err)))?;
        let res = &response
            .text()
            .await
            .map_err(|err| BybitError::Unknown(format!("{}", err)))?;
        let res = serde_json::from_str::<BybitHttpResponse>(res)
            .map_err(|err| BybitError::DeserializeError(format!("{res}: {err}")))?;
        match res.ret_code {
            0 => {
                let value_str = &res.result.to_string();
                let res = serde_json::from_str::<T>(value_str)
                    .map_err(|err| BybitError::DeserializeError(format!("{value_str}: {err}")))?;
                Ok(res)
            }
            _ => Err(BybitError::ApiError(res.ret_code, res.ret_msg)),
        }
    }

    async fn send_post_request<T>(
        &self,
        endpoint: &str,
        params: Map<String, Value>,
    ) -> BybitResult<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let signature = self.generate_post_signature(&timestamp, &params)?;

        let builder = self
            .client
            .post(format!("{BYBIT_HOST}/{}", endpoint))
            .json(&params);
        let builder = self.add_headers(builder, &timestamp, &signature);
        let response = builder
            .send()
            .await
            .map_err(|err| BybitError::Unknown(format!("{}", err)))?;
        let res = &response
            .text()
            .await
            .map_err(|err| BybitError::Unknown(format!("{}", err)))?;
        let res = serde_json::from_str::<BybitHttpResponse>(res)
            .map_err(|err| BybitError::DeserializeError(format!("{res}: {err}")))?;
        match res.ret_code {
            0 => {
                let value_str = &res.result.to_string();
                let res = serde_json::from_str::<T>(value_str)
                    .map_err(|err| BybitError::DeserializeError(format!("{value_str}: {err}")))?;
                Ok(res)
            }
            _ => Err(BybitError::ApiError(res.ret_code, res.ret_msg)),
        }
    }

    pub async fn get_open_order(
        &self,
        symbol: &String,
        cloid: Option<String>,
    ) -> BybitResult<GetOrderResponse> {
        let mut params = HashMap::new();
        params.insert("category", "linear");
        params.insert("symbol", symbol);
        params.insert("settleCoin", "USDT");
        let cloid_value: String;
        if let Some(cloid) = cloid {
            cloid_value = cloid;
            params.insert("orderLinkId", cloid_value.as_str());
        }

        self.send_get_request::<GetOrderResponse>("v5/order/realtime", params, true)
            .await
    }

    pub async fn place_order(
        &self,
        cloid: &String,
        symbol: &String,
        side: &String,
        price: &String,
        qty: &String,
    ) -> BybitResult<OrderResponse> {
        let mut params = Map::new();
        params.insert("orderLinkId".to_string(), json!(cloid));
        params.insert("category".to_string(), json!("linear"));
        params.insert("symbol".to_string(), json!(symbol));
        params.insert("side".to_string(), json!(side));
        params.insert("positionIdx".to_string(), json!(0));
        params.insert("orderType".to_string(), json!("Limit"));
        params.insert("qty".to_string(), json!(qty));
        params.insert("price".to_string(), json!(price));
        params.insert("timeInForce".to_string(), json!("GTC"));
        debug!("placing order with parameters {:?}", params);

        self.send_post_request::<OrderResponse>("v5/order/create", params)
            .await
    }

    pub async fn cancel_order(
        &self,
        symbol: &String,
        cloid: &String,
    ) -> BybitResult<OrderResponse> {
        let mut params = Map::new();
        params.insert("category".to_string(), json!("linear"));
        params.insert("symbol".to_string(), json!(symbol));
        params.insert("orderLinkId".to_string(), json!(cloid));

        self.send_post_request::<OrderResponse>("v5/order/cancel", params)
            .await
    }

    pub async fn cancel_all_orders(&self, symbol: &String) -> BybitResult<CancelAllOrdersResponse> {
        let mut params = Map::new();
        params.insert("category".to_string(), json!("linear"));
        params.insert("symbol".to_string(), json!(symbol));

        self.send_post_request::<CancelAllOrdersResponse>("v5/order/cancel-all", params)
            .await
    }

    pub async fn amend_order(
        &self,
        symbol: &String,
        cloid: &String,
        price: &String,
        // qty: &String,
    ) -> BybitResult<OrderResponse> {
        let mut params = Map::new();
        params.insert("category".to_string(), json!("linear"));
        params.insert("orderLinkId".to_string(), json!(cloid));
        params.insert("symbol".to_string(), json!(symbol));
        params.insert("price".to_string(), json!(price));
        // params.insert("qty".to_string(), json!(qty));
        debug!("amending order with parameters {:?}", params);

        self.send_post_request::<OrderResponse>("v5/order/amend", params)
            .await
    }

    pub async fn get_orderbook(&self, symbol: &String) -> BybitResult<OrderbookResponse> {
        let mut params = HashMap::new();
        params.insert("category", "linear");
        params.insert("symbol", symbol);
        params.insert("limit", "5");

        self.send_get_request::<OrderbookResponse>("v5/market/orderbook", params, false)
            .await
    }

    pub async fn get_positions(&self) -> BybitResult<GetPositionResponse> {
        let mut params = HashMap::new();
        params.insert("category", "linear");
        params.insert("settleCoin", "USDT");
        params.insert("limit", "200");

        self.send_get_request::<GetPositionResponse>("v5/position/list", params, true)
            .await
    }

    pub async fn get_tickers(&self) -> BybitResult<GetTickersResponse> {
        let mut params = HashMap::new();
        params.insert("category", "linear");

        self.send_get_request::<GetTickersResponse>("v5/market/tickers", params, false)
            .await
    }

    pub async fn get_wallet_balance(&self) -> BybitResult<GetWalletBalanceResponse> {
        // https://bybit-exchange.github.io/docs/v5/account/wallet-balance
        let mut params = HashMap::new();
        params.insert("accountType", "UNIFIED");

        self.send_get_request::<GetWalletBalanceResponse>("v5/account/wallet-balance", params, true)
            .await
    }

    pub async fn get_account_info(&self) -> BybitResult<BybitAccountInfo> {
        self.send_get_request::<BybitAccountInfo>("v5/account/info", HashMap::new(), true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn test_connector_get_tickers() {
    //     let client = BybitHttp::new("".into(), "".into());
    //     let tickers = client.get_tickers().await.unwrap();
    //     println!("{:?}", tickers);
    // }

    // #[tokio::test]
    // async fn get_account_info() {
    //     let client = BybitHttp::new("".into(), "".into());
    //     let account_info = client.get_account_info().await.unwrap();
    //     println!("{:?}", account_info);
    // }

    #[test]
    fn test_deserialize() {
        let res = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "orderId": "1616024329462743808",
                        "orderLinkId": "1616024329462743809"
                    },
                    {
                        "orderId": "1616024287544869632",
                        "orderLinkId": "1616024287544869633"
                    }
                ],
                "success": "1"
            },
            "retExtInfo": {},
            "time": 1707381118116
        }"#;
        let res = serde_json::from_str::<BybitHttpResponse>(res).unwrap();
        match res.ret_code {
            0 => {
                let value_str = &res.result.to_string();
                let res = serde_json::from_str::<CancelAllOrdersResponse>(value_str).unwrap();
                println!("{:?}", res.list)
            }
            _ => panic!(""),
        }
    }
}
