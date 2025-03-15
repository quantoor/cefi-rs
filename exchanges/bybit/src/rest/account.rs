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
