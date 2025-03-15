use async_trait::async_trait;

use crate::types::Orderbook;

#[async_trait]
pub trait InterfaceHttp {
    async fn get_server_time(&self) -> anyhow::Result<u64>;

    async fn get_orderbook(&self, symbol: &String, limit: Option<i32>)
    -> anyhow::Result<Orderbook>;
}

#[cfg(test)]
mod tests {
    use crate::exchanges::{binance::BinanceHttpWrapper, bybit::BybitHttpWrapper};

    use super::*;

    #[tokio::test]
    async fn test_get_server_time() {
        let mut exchanges: Vec<Box<dyn InterfaceHttp>> = vec![];

        let bybit = BybitHttpWrapper::new("".to_string(), "".to_string());
        exchanges.push(Box::new(bybit));

        let binance = BinanceHttpWrapper::new("".to_string(), "".to_string());
        exchanges.push(Box::new(binance));

        for exchange in exchanges {
            let server_time = exchange.get_server_time().await.unwrap();
            println!("server_time: {:?}", server_time);
        }
    }
}
