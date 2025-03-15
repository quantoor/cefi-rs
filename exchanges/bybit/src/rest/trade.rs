use std::collections::HashMap;

use serde_json::{Map, json};

use crate::{
    errors::BybitResult,
    http::BybitHttp,
    types::{CancelAllOrdersResponse, GetOrderResponse, OrderResponse},
};

impl BybitHttp {
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

        self.send_post_request::<OrderResponse>("v5/order/amend", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> BybitResult<()> {
        let api_key = std::env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY");
        let api_secret = std::env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET");
        let bybit = BybitHttp::new(api_key.to_string(), api_secret.to_string());

        // let account_info = bybit.get_account_info().await?;
        // println!("{:?}", account_info);

        // println!(
        //     "{:?}",
        //     bybit.get_open_order(&"SOLUSDT".to_string(), None).await?
        // );

        // println!("{:?}", bybit.get_orderbook(&"SOLUSDT".to_string()).await?);
        // println!("{:?}", bybit.get_orderbook(&"SOLUSDTs".to_string()).await?);

        println!(
            "{:?}",
            bybit
                .amend_order(
                    &"SOLUSDsT".to_string(),
                    &"123".to_string(),
                    &"123.74".to_string()
                )
                .await?
        );

        Ok(())
    }
}
