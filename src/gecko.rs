use chrono::prelude::*;
use reqwest;
use serde_json::Value;


pub async fn get_cardano_history(date: i64) -> Result<Value, reqwest::Error> {
    let url = "https://api.coingecko.com/api/v3/coins/cardano/history";
    let localization = "false";

    let date_time = DateTime::<Utc>::from_timestamp(date, 0).unwrap();
    let formatted_date = date_time.format("%d-%m-%Y").to_string();

    let url_with_params = format!(
        "{}?date={}&localization={}",
        url, formatted_date, localization
    );

    let response = reqwest::get(url_with_params).await?;

    let json = response.json::<serde_json::Value>().await?;
    // print!("JSON: {:#?}", &json);

    Ok(json)
}
