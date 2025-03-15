use std::collections::HashMap;

use crate::{errors::BybitResult, http::BybitHttp, types::GetPositionResponse};

impl BybitHttp {
    pub async fn get_positions(&self) -> BybitResult<GetPositionResponse> {
        let mut params = HashMap::new();
        params.insert("category", "linear");
        params.insert("settleCoin", "USDT");
        params.insert("limit", "200");

        self.send_get_request::<GetPositionResponse>("v5/position/list", params, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_positions() -> BybitResult<()> {
        dotenv().ok();
        let api_key = std::env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY");
        let api_secret = std::env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET");
        let client = BybitHttp::new(api_key.to_string(), api_secret.to_string());
        let res = client.get_positions().await;
        println!("{:?}", res);
        Ok(())
    }
}
