use std::collections::HashMap;

use crate::{
    errors::BybitResult,
    http::BybitHttp,
    types::{BybitAccountInfo, GetWalletBalanceResponse},
};

pub async fn get_wallet_balance(client: &BybitHttp) -> BybitResult<GetWalletBalanceResponse> {
    let mut params = HashMap::new();
    params.insert("accountType", "UNIFIED");

    client
        .send_get_request::<GetWalletBalanceResponse>("v5/account/wallet-balance", params, true)
        .await
}

pub async fn get_account_info(client: &BybitHttp) -> BybitResult<BybitAccountInfo> {
    client
        .send_get_request::<BybitAccountInfo>("v5/account/info", HashMap::new(), true)
        .await
}
