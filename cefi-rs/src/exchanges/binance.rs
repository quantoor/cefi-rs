use binance::http::BinanceHttp;

use crate::interface_http::InterfaceHttp;

impl InterfaceHttp for BinanceHttp {
    fn get_server_time(&self) -> anyhow::Result<u64> {
        todo!()
    }
}
