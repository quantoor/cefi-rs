use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(strum_macros::Display, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BybitSide {
    Buy,
    Sell,
    #[serde(rename = "")]
    Flat,
}

#[derive(strum_macros::Display, Serialize, Deserialize, Debug, Clone)]
pub enum BybitOrderStatus {
    New,
    PartiallyFilled,
    Untriggered,
    Rejected,
    PartiallyFilledCanceled,
    Filled,
    Cancelled,
    Triggered,
    Deactivated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BybitHttpResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: Value,
    pub ret_ext_info: Value,
    pub time: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    pub list: Vec<OrderResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponseData {
    pub order_id: String,
    pub order_link_id: String,
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub side: BybitSide,
    pub order_status: BybitOrderStatus,
    pub create_type: String,
    pub cancel_type: String,
    pub reject_reason: String,
    pub leaves_qty: String,
    pub cum_exec_qty: String,
    pub created_time: u64,
    pub updated_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub next_page_cursor: String,
    pub category: String,
    pub list: Vec<GetOrderResponseData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponse {
    pub s: String,
    pub b: Vec<Vec<String>>, // [["price", "amount"]]
    pub a: Vec<Vec<String>>, // [["price", "amount"]]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionResponse {
    pub category: String,
    pub list: Vec<BybitLinearPosition>,
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BybitLinearPosition {
    // "avgPrice": "143.87891359",
    // "bustPrice": "0.010",
    pub created_time: String,
    // "cumRealisedPnl": "-3.18147417",
    // "curRealisedPnl": "-3.67493931",
    pub leverage: String,
    // "leverageSysUpdatedTime": "",
    pub liq_price: String,
    pub mark_price: String,
    // "mmrSysUpdatedTime": "",
    pub position_balance: String,
    #[serde(rename = "positionIM")]
    pub position_im: String,
    pub position_idx: u8,
    #[serde(rename = "positionMM")]
    pub position_mm: String,
    pub position_status: String,
    pub position_value: String,
    // "riskId": 281,
    pub risk_limit_value: String,
    // "seq": 153117882808,
    pub side: BybitSide,
    pub size: String,
    pub symbol: String,
    // "tpslMode": "Full",
    // "tradeMode": 0,
    pub unrealised_pnl: String,
    pub updated_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTickersResponse {
    pub category: String,
    pub list: Vec<BybitTicker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BybitTicker {
    pub symbol: String,
    pub mark_price: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BybitAccountInfo {
    pub unified_margin_status: u8,
    pub margin_mode: String,
    pub is_master_trader: bool,
    pub spot_hedging_status: String,
    pub updated_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletBalanceResponse {
    pub list: Vec<WalletBalance>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    pub account_type: String,
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: String,
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: String,
    pub total_equity: String,
    pub total_wallet_balance: String,
    pub total_margin_balance: String,
    pub total_available_balance: String,
    #[serde(rename = "totalPerpUPL")]
    pub total_perp_upl: String,
    pub total_initial_margin: String,
    pub total_maintenance_margin: String,
    pub coin: Vec<WalletBalanceCoin>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceCoin {
    pub coin: String,
    pub equity: String,
    pub usd_value: String,
    pub wallet_balance: String,
    // pub free: Option<String>, // only for SPOT
    pub locked: String,
    pub spot_hedging_qty: String,
    pub borrow_amount: String,
    pub available_to_withdraw: String,
    pub accrued_interest: String,
    #[serde(rename = "totalOrderIM")]
    pub total_order_im: String,
    #[serde(rename = "totalPositionIM")]
    pub total_position_im: String,
    #[serde(rename = "totalPositionMM")]
    pub total_position_mm: String,
    pub unrealised_pnl: Option<String>,
    pub cum_realised_pnl: Option<String>,
    pub bonus: Option<String>, // only for UNIFIED
    pub margin_collateral: bool,
    pub collateral_switch: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_http_response_error() {
        let json_data = r#"{"retCode":10001,"retMsg":"Qty invalid","result":{},"retExtInfo":{},"time":1727663049561}"#;
        let r = serde_json::from_str::<BybitHttpResponse>(&json_data).unwrap();
        println!("{:?}", r.result.to_string());
    }

    #[test]
    fn deserialize_http_order_response() {
        let json_data = r#"{"retCode":0,"retMsg":"OK","result":{"orderId":"29c242ff-7da2-47d4-a243-ba6ff034005e","orderLinkId":""},"retExtInfo":{},"time":1727663866609}"#;
        let r = serde_json::from_str::<BybitHttpResponse>(&json_data).unwrap();
        println!("{:?}", r.result.to_string());
    }

    #[test]
    fn test_deserialize_get_positions_response() {
        let response = r#"{"category":"linear","list":[{"adlRankIndicator":2,"autoAddMargin":0,"avgPrice":"149.127428","bustPrice":"0.010","createdTime":"1715069854842","cumRealisedPnl":"-0.86731017","curRealisedPnl":"-1.36077531","isReduceOnly":false,"leverage":"10","leverageSysUpdatedTime":"","liqPrice":"","markPrice":"173.732","mmrSysUpdatedTime":"","positionBalance":"7.49328044","positionIM":"0.9916974","positionIdx":0,"positionMM":"0.0000325","positionStatus":"Normal","positionValue":"74.563714","riskId":281,"riskLimitValue":"210000","seq":155916379358,"sessionAvgPrice":"","side":"Buy","size":"0.5","stopLoss":"0.000","symbol":"SOLUSDT","takeProfit":"0.000","tpslMode":"Full","tradeMode":0,"trailingStop":"0.000","unrealisedPnl":"12.302286","updatedTime":"1730087396775"},{"adlRankIndicator":2,"autoAddMargin":0,"avgPrice":"1.3841","bustPrice":"15.22880","createdTime":"1724910435941","cumRealisedPnl":"-5.7101154","curRealisedPnl":"0.38234475","isReduceOnly":false,"leverage":"10","leverageSysUpdatedTime":"","liqPrice":"15.21490","markPrice":"1.69352","mmrSysUpdatedTime":"","positionBalance":"4.48667381","positionIM":"0.27682","positionIdx":0,"positionMM":"1.52288","positionStatus":"Normal","positionValue":"13.841","riskId":1,"riskLimitValue":"200000","seq":184120255016,"sessionAvgPrice":"","side":"Sell","size":"10","stopLoss":"0.00000","symbol":"SUIUSDT","takeProfit":"0.00000","tpslMode":"Full","tradeMode":0,"trailingStop":"0.00000","unrealisedPnl":"-3.0942","updatedTime":"1730087396775"},{"adlRankIndicator":2,"autoAddMargin":0,"avgPrice":"1.6729","bustPrice":"36.0273","createdTime":"1723618141587","cumRealisedPnl":"0.11741437","curRealisedPnl":"0.11741437","isReduceOnly":false,"leverage":"10","leverageSysUpdatedTime":"","liqPrice":"36.0161","markPrice":"2.3678","mmrSysUpdatedTime":"","positionBalance":"3.45280842","positionIM":"0.08899828","positionIdx":0,"positionMM":"0.96553164","positionStatus":"Normal","positionValue":"6.6916","riskId":1,"riskLimitValue":"100000","seq":135593215329,"sessionAvgPrice":"","side":"Sell","size":"4","stopLoss":"0.0000","symbol":"WIFUSDT","takeProfit":"0.0000","tpslMode":"Full","tradeMode":0,"trailingStop":"0.0000","unrealisedPnl":"-2.7796","updatedTime":"1730087396775"},{"adlRankIndicator":4,"autoAddMargin":0,"avgPrice":"0.116144","bustPrice":"0.439317","createdTime":"1723618019735","cumRealisedPnl":"0.30966853","curRealisedPnl":"0.30966853","isReduceOnly":false,"leverage":"10","leverageSysUpdatedTime":"","liqPrice":"0.434671","markPrice":"0.063599","mmrSysUpdatedTime":"","positionBalance":"5.02440687","positionIM":"3.9953536","positionIdx":0,"positionMM":"7.5562524","positionStatus":"Normal","positionValue":"49.94192","riskId":1,"riskLimitValue":"25000","seq":30747967432,"sessionAvgPrice":"","side":"Sell","size":"430","stopLoss":"0.000000","symbol":"MOTHERUSDT","takeProfit":"0.000000","tpslMode":"Full","tradeMode":0,"trailingStop":"0.000000","unrealisedPnl":"22.59435","updatedTime":"1730082173992"}],"nextPageCursor":""}"#;
        let res = serde_json::from_str::<GetPositionResponse>(response).unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn test_deserialize_get_account_info() {
        let response = "{\"marginMode\":\"REGULAR_MARGIN\",\"updatedTime\":\"0\",\"unifiedMarginStatus\":1,\"dcpStatus\":\"OFF\",\"timeWindow\":0,\"smpGroup\":0,\"isMasterTrader\":false,\"spotHedgingStatus\":\"OFF\"}";
        let res = serde_json::from_str::<BybitAccountInfo>(response).unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn test_deserialize_get_wallet_balance() {
        let response = "{\"list\":[{\"totalEquity\":\"233057.94428155\",\"accountIMRate\":\"0.5492\",\"totalMarginBalance\":\"227393.49283372\",\"totalInitialMargin\":\"124896.73199212\",\"accountType\":\"UNIFIED\",\"totalAvailableBalance\":\"102496.7608416\",\"accountMMRate\":\"0.4201\",\"totalPerpUPL\":\"-54464.69617576\",\"totalWalletBalance\":\"281858.18900948\",\"accountLTV\":\"0.1807\",\"totalMaintenanceMargin\":\"95534.19984428\",\"coin\":[{\"availableToBorrow\":\"\",\"bonus\":\"0\",\"accruedInterest\":\"0\",\"availableToWithdraw\":\"0.99961\",\"totalOrderIM\":\"\",\"equity\":\"0.99961\",\"totalPositionMM\":\"\",\"usdValue\":\"0.9996\",\"unrealisedPnl\":\"0\",\"collateralSwitch\":true,\"spotHedgingQty\":\"0\",\"borrowAmount\":\"0.000000000000000000\",\"totalPositionIM\":\"\",\"walletBalance\":\"0.99961\",\"cumRealisedPnl\":\"0\",\"locked\":\"0\",\"marginCollateral\":true,\"coin\":\"USDC\"},{\"availableToBorrow\":\"\",\"bonus\":\"0\",\"accruedInterest\":\"0\",\"availableToWithdraw\":\"1.04517272\",\"totalOrderIM\":\"\",\"equity\":\"2.83028992\",\"totalPositionMM\":\"\",\"usdValue\":\"283221.95954498\",\"unrealisedPnl\":\"0\",\"collateralSwitch\":true,\"spotHedgingQty\":\"0\",\"borrowAmount\":\"0.000000000000000000\",\"totalPositionIM\":\"\",\"walletBalance\":\"2.83028992\",\"cumRealisedPnl\":\"-0.00000008\",\"locked\":\"0\",\"marginCollateral\":true,\"coin\":\"BTC\"},{\"availableToBorrow\":\"\",\"bonus\":\"0\",\"accruedInterest\":\"1.91429833\",\"availableToWithdraw\":\"0\",\"totalOrderIM\":\"\",\"equity\":\"-50119.76896898\",\"totalPositionMM\":\"\",\"usdValue\":\"-50165.02712036\",\"unrealisedPnl\":\"-54415.55892605\",\"collateralSwitch\":true,\"spotHedgingQty\":\"0\",\"borrowAmount\":\"50119.768968985806327169\",\"totalPositionIM\":\"\",\"walletBalance\":\"4295.78995706\",\"cumRealisedPnl\":\"-221545.17712055\",\"locked\":\"0\",\"marginCollateral\":true,\"coin\":\"USDT\"},{\"availableToBorrow\":\"\",\"bonus\":\"0\",\"accruedInterest\":\"0\",\"availableToWithdraw\":\"0.008\",\"totalOrderIM\":\"\",\"equity\":\"0.008\",\"totalPositionMM\":\"\",\"usdValue\":\"0.01223879\",\"unrealisedPnl\":\"0\",\"collateralSwitch\":false,\"spotHedgingQty\":\"0\",\"borrowAmount\":\"0.000000000000000000\",\"totalPositionIM\":\"\",\"walletBalance\":\"0.008\",\"cumRealisedPnl\":\"0\",\"locked\":\"0\",\"marginCollateral\":true,\"coin\":\"POPCAT\"},{\"availableToBorrow\":\"\",\"bonus\":\"0\",\"accruedInterest\":\"0\",\"availableToWithdraw\":\"0.00129138\",\"totalOrderIM\":\"\",\"equity\":\"0.00129138\",\"totalPositionMM\":\"\",\"usdValue\":\"0.00001813\",\"unrealisedPnl\":\"0\",\"collateralSwitch\":false,\"spotHedgingQty\":\"0\",\"borrowAmount\":\"0.000000000000000000\",\"totalPositionIM\":\"\",\"walletBalance\":\"0.00129138\",\"cumRealisedPnl\":\"0\",\"locked\":\"0\",\"marginCollateral\":true,\"coin\":\"BLAST\"}]}]}";
        let res = serde_json::from_str::<GetWalletBalanceResponse>(response).unwrap();
        println!("{:?}", res);
    }
}
