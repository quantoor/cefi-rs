use futures::stream::SplitStream;
use futures::{SinkExt, StreamExt};
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{error, info, warn};

use crate::ws_types::BybitWsUpdate;

static BYBIT_HOST: &'static str = "wss://stream.bybit.com/v5/private";

type WsStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthMessage {
    req_id: Option<String>,
    op: String,
    args: Vec<String>,
}

fn get_auth_message(api_key: String, api_secret: String) -> String {
    let expires = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        + 5000;
    let mut mac =
        Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).expect("Invalid key length");
    mac.update(format!("GET/realtime{expires}").as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());
    let auth_msg = AuthMessage {
        req_id: None,
        op: "auth".to_string(),
        args: vec![api_key.to_string(), expires.to_string(), signature],
    };
    serde_json::to_string(&auth_msg).unwrap()
}

pub struct BybitWs {
    stream: WsStream,
}

impl BybitWs {
    pub async fn new(api_key: String, api_secret: String, topics: Vec<String>) -> Self {
        let (ws_stream, _) = connect_async(BYBIT_HOST).await.expect("Failed to connect");

        let (mut write, read) = ws_stream.split();

        // send auth message
        let auth_message = get_auth_message(api_key, api_secret);
        write
            .send(Message::Text(auth_message))
            .await
            .expect("error sending auth message");

        // subscribe to private updates
        let sub_msg = json!({
            "req_id": "1",
            "op": "subscribe",
            "args": topics
        });
        write
            .send(Message::Text(sub_msg.to_string()))
            .await
            .unwrap_or_else(|err| error!("Error sending ping: {err}"));

        tokio::spawn(async move {
            loop {
                info!("Sending ping...");
                let msg = json!({
                    "req_id": "100001",
                    "op": "ping",
                });
                write
                    .send(Message::Text(msg.to_string()))
                    .await
                    .unwrap_or_else(|err| error!("Error sending ping: {err}"));
                tokio::time::sleep(Duration::from_secs(20)).await;
            }
        });

        Self { stream: read }
    }

    pub async fn next(&mut self) -> Option<BybitWsUpdate> {
        if let Some(msg) = self.stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<BybitWsUpdate>(text.as_str()) {
                        Ok(update) => return Some(update),
                        Err(err) => {
                            error!("error deserializing {text}: {err}");
                            return None;
                        }
                    }
                }
                Ok(Message::Binary(bin)) => {
                    info!("Received binary data: {:?}", bin);
                }
                Ok(Message::Ping(ping)) => {
                    info!("Received ping: {:?}", ping);
                }
                Ok(Message::Pong(_pong)) => {
                    info!("Received pong");
                }
                Ok(Message::Close(close)) => {
                    warn!("Connection closed: {:?}", close);
                }
                Err(e) => {
                    error!("Error receiving message: {:?}", e);
                }
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::ws_types::BybitWsUpdate;

    #[test]
    fn test_deserialize_position() {
        let res = r#"{
        "id": "48746281_position.linear_1737878863176",
        "topic": "position.linear",
        "creationTime": 1737878863176,
        "data": [
            {
            "positionIdx": 0,
            "tradeMode": 0,
            "riskId": 11,
            "riskLimitValue": "900000",
            "symbol": "ETHUSDT",
            "side": "",
            "size": "0",
            "entryPrice": "0",
            "sessionAvgPrice": "",
            "leverage": "10",
            "positionValue": "0",
            "positionBalance": "0",
            "markPrice": "3342.31",
            "positionIM": "0",
            "positionMM": "0",
            "takeProfit": "0",
            "stopLoss": "0",
            "trailingStop": "0",
            "unrealisedPnl": "0",
            "cumRealisedPnl": "-323.44165063",
            "curRealisedPnl": "0",
            "createdTime": "1716645225709",
            "updatedTime": "1717818863076",
            "tpslMode": "Full",
            "liqPrice": "",
            "bustPrice": "",
            "category": "linear",
            "positionStatus": "Normal",
            "adlRankIndicator": 0,
            "autoAddMargin": 0,
            "leverageSysUpdatedTime": "",
            "mmrSysUpdatedTime": "",
            "seq": 213418534265,
            "isReduceOnly": false
            }
        ]
        }"#;
        let res = serde_json::from_str::<BybitWsUpdate>(res).unwrap();
        println!("{:?}", res)
    }
}
