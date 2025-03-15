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
