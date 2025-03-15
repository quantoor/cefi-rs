use bybit::http::BybitHttp;

use crate::interface_http::InterfaceHttp;

impl InterfaceHttp for BybitHttp {
    fn get_server_time(&self) -> anyhow::Result<u64> {
        todo!()
    }
}
