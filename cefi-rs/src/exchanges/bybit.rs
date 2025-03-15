use async_trait::async_trait;
use bybit::{http::BybitHttp, rest::market::get_server_time};

use crate::interface_http::InterfaceHttp;

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
}
