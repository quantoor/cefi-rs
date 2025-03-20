pub type ExchangeSymbol = String;

pub struct PlaceOrderParams {
    pub symbol: ExchangeSymbol,
    pub is_buy: bool,
    pub price: f64,
    pub amount: f64,
}

pub struct PlaceOrderResponse {
    pub order_id: String,
    pub order_link_id: String,
}

pub struct CancelOrderResponse {}

pub struct CancelAllOrdersResponse {}

pub struct AmendOrderParams {}

pub struct AmendOrderResponse {}
