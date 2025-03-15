use std::collections::HashMap;

use crate::{
    errors::BybitResult,
    http::BybitHttp,
    types::{BybitAccountInfo, GetWalletBalanceResponse},
};

impl BybitHttp {
    pub async fn get_wallet_balance(&self) -> BybitResult<GetWalletBalanceResponse> {
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
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_wallet_balance() {
        dotenv().ok();
        let api_key = std::env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY");
        let api_secret = std::env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET");
        let client = BybitHttp::new(api_key, api_secret);
        let balance = client.get_wallet_balance().await.unwrap();
        println!("{:?}", balance);
    }

    #[tokio::test]
    async fn test_get_account_info() {
        dotenv().ok();
        let api_key = std::env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY");
        let api_secret = std::env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET");
        let client = BybitHttp::new(api_key, api_secret);
        let account_info = client.get_account_info().await.unwrap();
        println!("{:?}", account_info);
    }
}
