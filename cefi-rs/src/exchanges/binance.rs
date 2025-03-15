use crate::{
    interface_http::InterfaceHttp,
    trade::*,
    types::{Orderbook, OrderbookLevel},
};
use async_trait::async_trait;
use binance::{
    http::BinanceHttp,
    rest::market::{check_server_time, get_orderbook},
    types::OrderBook as BinanceOrderBook,
};

pub struct BinanceHttpWrapper {
    http: BinanceHttp,
}

impl BinanceHttpWrapper {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            http: BinanceHttp::new(api_key, api_secret),
        }
    }
}

#[async_trait]
impl InterfaceHttp for BinanceHttpWrapper {
    async fn get_server_time(&self) -> anyhow::Result<u64> {
        let server_time = check_server_time(&self.http)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(server_time.server_time)
    }

    async fn get_orderbook(
        &self,
        symbol: &String,
        limit: Option<i32>,
    ) -> anyhow::Result<Orderbook> {
        let orderbook = get_orderbook(&self.http, symbol, limit)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        todo!()
        // Ok(orderbook)
    }

    async fn place_order(&self, params: &PlaceOrderParams) -> anyhow::Result<PlaceOrderResponse> {
        todo!()
    }

    async fn cancel_order(&self, order_id: &String) -> anyhow::Result<CancelOrderResponse> {
        todo!()
    }

    async fn cancel_all_orders(&self, symbol: &String) -> anyhow::Result<CancelAllOrdersResponse> {
        todo!()
    }

    async fn amend_order(
        &self,
        order_id: &String,
        params: &AmendOrderParams,
    ) -> anyhow::Result<AmendOrderResponse> {
        todo!()
    }
}

impl Orderbook {
    fn from_binance_orderbook(orderbook: BinanceOrderBook, symbol: String) -> Self {
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
            timestamp_ms: orderbook.event_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::exchanges::binance::BinanceHttpWrapper;

    use super::*;

    #[tokio::test]
    async fn test_get_orderbook() {
        let binance = BinanceHttpWrapper::new("".to_string(), "".to_string());
        let orderbook = binance
            .get_orderbook(&"BTCUSDT".to_string(), Some(5))
            .await
            .unwrap();
        println!("orderbook: {:?}", orderbook);
    }
}
