use crate::{
    errors::{BybitError, BybitResult},
    types::*,
};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{Client, RequestBuilder};
use serde_json::{Map, Value};
use sha2::Sha256;
use std::collections::HashMap;

static BYBIT_HOST: &'static str = "https://api.bybit.com";
static RECV_WINDOW: &'static str = "5000";

pub struct BybitHttp {
    api_key: String,
    api_secret: String,
    client: Client,
}

impl BybitHttp {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("reqwest client"),
        }
    }

    fn generate_get_signature(
        &self,
        timestamp: &str,
        params: &HashMap<&str, &str>,
    ) -> BybitResult<String> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(timestamp.as_bytes());
        mac.update(self.api_key.as_bytes());
        mac.update(RECV_WINDOW.as_bytes());
        mac.update(Self::generate_query_str(params).as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        Ok(hex::encode(code_bytes))
    }

    fn generate_post_signature(
        &self,
        timestamp: &str,
        params: &Map<String, Value>,
    ) -> BybitResult<String> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(timestamp.as_bytes());
        mac.update(self.api_key.as_bytes());
        mac.update(RECV_WINDOW.as_bytes());
        mac.update(serde_json::to_string(&params)?.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        Ok(hex::encode(code_bytes))
    }

    fn generate_query_str(params: &HashMap<&str, &str>) -> String {
        params
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&")
    }

    fn add_headers(
        &self,
        builder: RequestBuilder,
        timestamp: &String,
        signature: &String,
    ) -> RequestBuilder {
        builder
            .header("X-BAPI-API-KEY", self.api_key.clone())
            .header("X-BAPI-SIGN", signature)
            .header("X-BAPI-SIGN-TYPE", "2")
            .header("X-BAPI-TIMESTAMP", timestamp.clone())
            .header("X-BAPI-RECV-WINDOW", RECV_WINDOW)
            .header("Content-Type", "application/json")
    }

    pub(crate) async fn send_get_request<T>(
        &self,
        endpoint: &str,
        params: HashMap<&str, &str>,
        is_auth: bool,
    ) -> BybitResult<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        let query_str = Self::generate_query_str(&params);

        let mut builder = self
            .client
            .get(&format!("{}/{}?{}", BYBIT_HOST, endpoint, query_str));

        if is_auth {
            let timestamp = Utc::now().timestamp_millis().to_string();
            let signature = self.generate_get_signature(&timestamp, &params)?;
            builder = self.add_headers(builder, &timestamp, &signature)
        }

        let response = builder
            .send()
            .await
            .map_err(|err| BybitError::Unknown(format!("{}", err)))?;
        let res = &response
            .text()
            .await
            .map_err(|err| BybitError::Unknown(format!("{}", err)))?;
        let res = serde_json::from_str::<BybitHttpResponse>(res)
            .map_err(|err| BybitError::DeserializeError(format!("{res}: {err}")))?;
        match res.ret_code {
            0 => {
                let value_str = &res.result.to_string();
                let res = serde_json::from_str::<T>(value_str)
                    .map_err(|err| BybitError::DeserializeError(format!("{value_str}: {err}")))?;
                Ok(res)
            }
            _ => Err(BybitError::ApiError(res.ret_code, res.ret_msg)),
        }
    }

    pub(crate) async fn send_post_request<T>(
        &self,
        endpoint: &str,
        params: Map<String, Value>,
    ) -> BybitResult<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let signature = self.generate_post_signature(&timestamp, &params)?;

        let builder = self
            .client
            .post(format!("{BYBIT_HOST}/{}", endpoint))
            .json(&params);
        let builder = self.add_headers(builder, &timestamp, &signature);
        let response = builder
            .send()
            .await
            .map_err(|err| BybitError::Unknown(format!("{}", err)))?;
        let res = &response
            .text()
            .await
            .map_err(|err| BybitError::Unknown(format!("{}", err)))?;
        let res = serde_json::from_str::<BybitHttpResponse>(res)
            .map_err(|err| BybitError::DeserializeError(format!("{res}: {err}")))?;
        match res.ret_code {
            0 => {
                let value_str = &res.result.to_string();
                let res = serde_json::from_str::<T>(value_str)
                    .map_err(|err| BybitError::DeserializeError(format!("{value_str}: {err}")))?;
                Ok(res)
            }
            _ => Err(BybitError::ApiError(res.ret_code, res.ret_msg)),
        }
    }
}
