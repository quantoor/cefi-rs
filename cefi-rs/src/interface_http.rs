pub trait InterfaceHttp {
    fn get_server_time(&self) -> anyhow::Result<u64>;
}
