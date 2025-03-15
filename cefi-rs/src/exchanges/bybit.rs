use async_trait::async_trait;
use bybit::{http::BybitHttp, rest::market::get_server_time};

use crate::{interface_http::InterfaceHttp, types::Orderbook};

pub struct BybitHttpWrapper {
    http: BybitHttp,
}

impl BybitHttpWrapper {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            http: BybitHttp::new(api_key, api_secret),
        }
    }
}

#[async_trait]
impl InterfaceHttp for BybitHttpWrapper {
    async fn get_server_time(&self) -> anyhow::Result<u64> {
        let server_time = get_server_time(&self.http)
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
        todo!()
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
