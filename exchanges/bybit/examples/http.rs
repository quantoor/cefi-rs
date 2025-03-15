use bybit::{errors::BybitResult, http::BybitHttp};

#[tokio::main]
async fn main() -> BybitResult<()> {
    let api_key = std::env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY");
    let api_secret = std::env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET");
    let bybit = BybitHttp::new(api_key.to_string(), api_secret.to_string());

    // let account_info = bybit.get_account_info().await?;
    // println!("{:?}", account_info);

    // println!(
    //     "{:?}",
    //     bybit.get_open_order(&"SOLUSDT".to_string(), None).await?
    // );

    // println!("{:?}", bybit.get_orderbook(&"SOLUSDT".to_string()).await?);
    // println!("{:?}", bybit.get_orderbook(&"SOLUSDTs".to_string()).await?);

    println!(
        "{:?}",
        bybit
            .amend_order(
                &"SOLUSDsT".to_string(),
                &"123".to_string(),
                &"123.74".to_string()
            )
            .await?
    );

    Ok(())
}

//{"retCode":10001,"retMsg":"params error: symbol invalid","result":{},"retExtInfo":{},"time":1741615413686}
//{"retCode":0,"retMsg":"OK","result":{"s":"SOLUSDT","b":[["123.73","242.2"],["123.72","209.3"],["123.71","288"],["123.7","487.2"],["123.69","832.5"]],"a":[["123.74","8.5"],["123.75","275.9"],["123.76","386.9"],["123.77","379.9"],["123.78","965.2"]],"ts":1741615623678,"u":46138715,"seq":191265669802,"cts":1741615623677},"retExtInfo":{},"time":1741615623709}
