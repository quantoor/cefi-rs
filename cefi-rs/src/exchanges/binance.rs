use async_trait::async_trait;
use binance::{http::BinanceHttp, rest::market::check_server_time};

use crate::interface_http::InterfaceHttp;

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
}
