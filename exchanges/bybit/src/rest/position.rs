use std::collections::HashMap;

use crate::{errors::BybitResult, http::BybitHttp, types::GetPositionResponse};

pub async fn get_positions(client: &BybitHttp) -> BybitResult<GetPositionResponse> {
    let mut params = HashMap::new();
    params.insert("category", "linear");
    params.insert("settleCoin", "USDT");
    params.insert("limit", "200");

    client
        .send_get_request::<GetPositionResponse>("v5/position/list", params, true)
        .await
}
