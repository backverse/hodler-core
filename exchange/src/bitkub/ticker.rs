use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ticker {
  #[serde(rename = "lowestAsk")]
  pub ask_price: f32,
  #[serde(rename = "highestBid")]
  pub bid_price: f32,
  #[serde(rename = "stream")]
  pub ticker_name: String,
  #[serde(rename = "baseVolume")]
  pub volume: f32,
  #[serde(rename = "percentChange")]
  pub change: f32,
  #[serde(default = "get_current_timestamp")]
  pub timestamp: i64,
}

fn get_current_timestamp() -> i64 {
  Utc::now().timestamp_millis()
}
