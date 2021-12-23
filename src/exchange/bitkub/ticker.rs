use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
pub struct Ticker {
  #[serde(rename = "lowestAsk")]
  pub ask_price: f32,
  #[serde(rename = "highestBid")]
  pub bid_price: f32,
  pub stream: String,
}

impl Ticker {
  pub fn from_str(string: String) -> Ticker {
    from_str::<Option<Ticker>>(&string).unwrap().unwrap()
  }
}
