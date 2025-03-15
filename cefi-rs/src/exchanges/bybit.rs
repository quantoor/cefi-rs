use async_trait::async_trait;
use bybit::{http::BybitHttp, rest::market::OrderbookResponse};

use crate::{
    interface_http::InterfaceHttp,
    types::{Orderbook, OrderbookLevel},
};

pub struct BybitHttpWrapper {
    client: BybitHttp,
}

impl BybitHttpWrapper {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            client: BybitHttp::new(api_key, api_secret),
        }
    }
}

#[async_trait]
impl InterfaceHttp for BybitHttpWrapper {
    async fn get_server_time(&self) -> anyhow::Result<u64> {
        let server_time = self
            .client
            .get_server_time()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        let nanos = server_time.time_nano.parse::<u64>()?;

        Ok(nanos / 1_000_000)
    }

    async fn get_orderbook(
        &self,
        symbol: &String,
        limit: Option<i32>,
    ) -> anyhow::Result<Orderbook> {
        let orderbook = self
            .client
            .get_orderbook(
                "linear".to_string(),
                symbol.to_string(),
                limit.unwrap_or(10),
            )
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        Ok(Orderbook::from_bybit_orderbook(
            orderbook,
            symbol.to_string(),
        ))
    }

    async fn place_order(
        &self,
        params: &crate::trade::PlaceOrderParams,
    ) -> anyhow::Result<crate::trade::PlaceOrderResponse> {
        todo!()
    }

    async fn cancel_order(
        &self,
        order_id: &String,
    ) -> anyhow::Result<crate::trade::CancelOrderResponse> {
        todo!()
    }

    async fn cancel_all_orders(
        &self,
        symbol: &String,
    ) -> anyhow::Result<crate::trade::CancelAllOrdersResponse> {
        todo!()
    }

    async fn amend_order(
        &self,
        order_id: &String,
        params: &crate::trade::AmendOrderParams,
    ) -> anyhow::Result<crate::trade::AmendOrderResponse> {
        todo!()
    }
}

impl Orderbook {
    fn from_bybit_orderbook(orderbook: OrderbookResponse, symbol: String) -> Self {
        Orderbook {
            symbol: symbol,
            asks: orderbook
                .asks
                .into_iter()
                .map(|ask| OrderbookLevel {
                    price: ask[0].parse::<f64>().unwrap(),
                    amount: ask[1].parse::<f64>().unwrap(),
                })
                .collect(),
            bids: orderbook
                .bids
                .into_iter()
                .map(|bid| OrderbookLevel {
                    price: bid[0].parse::<f64>().unwrap(),
                    amount: bid[1].parse::<f64>().unwrap(),
                })
                .collect(),
            timestamp_ms: orderbook.timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::exchanges::bybit::BybitHttpWrapper;

    use super::*;

    #[tokio::test]
    async fn test_get_orderbook() {
        let bybit = BybitHttpWrapper::new("".to_string(), "".to_string());
        let orderbook = bybit
            .get_orderbook(&"BTCUSDT".to_string(), Some(10))
            .await
            .unwrap();
        println!("orderbook: {:?}", orderbook);
    }
}
