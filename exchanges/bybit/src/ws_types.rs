use serde::{Deserialize, Serialize};

use crate::types::{BybitLinearPosition, BybitOrderStatus, BybitSide};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BybitWsUpdate {
    AuthResponse(OpResponse),
    BybitUpdateData(BybitUpdateData),
    Pong(Pong),
}

#[derive(Deserialize, Debug)]
pub struct Pong {
    pub success: bool,
    pub ret_msg: String,
    pub op: String,
    pub conn_id: String,
}

#[derive(Deserialize, Debug)]
pub struct OpResponse {
    pub op: String,
    pub args: Vec<String>,
    pub conn_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "topic", content = "data")]
pub enum BybitUpdateData {
    #[serde(rename = "order.linear")]
    OrderLinear(Vec<BybitOrderLinearData>),
    #[serde(rename = "execution.linear")]
    ExecutionLinear(Vec<BybitExecutionLinearData>),
    #[serde(rename = "position.linear")]
    PositionLinear(Vec<BybitLinearPosition>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BybitOrderLinearData {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
    pub side: BybitSide,
    pub price: String,
    pub qty: String,
    pub order_status: BybitOrderStatus,
    pub create_type: String,
    pub cancel_type: String,
    pub reject_reason: String,
    pub leaves_qty: String,
    pub cum_exec_qty: String,
    pub created_time: String,
    pub updated_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BybitExecutionLinearData {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
    pub side: BybitSide,
    pub order_price: String,
    pub order_qty: String,
    pub leaves_qty: String,
    pub order_type: String,
    pub exec_fee: String,
    pub exec_id: String,
    pub exec_price: String,
    pub exec_qty: String,
    pub exec_type: String,
    pub exec_value: String,
    pub exec_time: String,
    pub is_maker: bool,
    pub fee_rate: String,
    pub mark_price: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_pong() {
        let json_data = r#"
        {"req_id":"100001","op":"pong","args":["1727434943071"],"conn_id":"cmjonqvavkfduu60h6d0-231ex0"}
        "#;
        let r = serde_json::from_str::<BybitWsUpdate>(&json_data).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_deserialize_auth_response() {
        let json_data = r#"
        {"success":true,"ret_msg":"","op":"auth","conn_id":"cmjoqsm8dkqdvjssdqvg-232sm4"}
        "#;
        let r = serde_json::from_str::<BybitWsUpdate>(&json_data).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_deserialize_order_linear() {
        let json_data = r#"
        {"topic":"order.linear","id":"62552242_SOLUSDT_149260954383","creationTime":1727433749096,"data":[{"category":"linear","symbol":"SOLUSDT","orderId":"f38c5e38-46ef-4a8a-b115-fd3392d4f144","orderLinkId":"","blockTradeId":"","side":"Buy","positionIdx":0,"orderStatus":"Cancelled","cancelType":"CancelByUser","rejectReason":"EC_PerCancelRequest","timeInForce":"GTC","isLeverage":"","price":"99","qty":"0.1","avgPrice":"","leavesQty":"0","leavesValue":"0","cumExecQty":"0","cumExecValue":"0","cumExecFee":"0","orderType":"Limit","stopOrderType":"","orderIv":"","triggerPrice":"","takeProfit":"","stopLoss":"","triggerBy":"","tpTriggerBy":"","slTriggerBy":"","triggerDirection":0,"placeType":"","lastPriceOnCreated":"156.17","closeOnTrigger":false,"reduceOnly":false,"smpGroup":0,"smpType":"None","smpOrderId":"","slLimitPrice":"0","tpLimitPrice":"0","tpslMode":"UNKNOWN","createType":"CreateByUser","marketUnit":"","createdTime":"1727433748480","updatedTime":"1727433749095","feeCurrency":""}]}
        "#;
        let r = serde_json::from_str::<BybitWsUpdate>(&json_data).unwrap();
        println!("{:?}", r);
    }
}
