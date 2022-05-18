use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ticker {
  #[serde(rename = "lowestAsk")]
  pub ask_price: f32,
  #[serde(rename = "highestBid")]
  pub bid_price: f32,
  #[serde(rename = "stream")]
  pub symbol: String,
}
