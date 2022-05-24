use serde::Deserialize;
use serde::Deserializer;
use std::str::FromStr;

#[derive(Debug)]
pub struct Ticker {
  pub ask_price: f32,
  pub bid_price: f32,
  pub ticker_name: String,
  pub change: f32,
  pub volume: f32,
  pub timestamp: i64,
}

#[derive(Deserialize)]
struct BinanceTicker {
  data: Data,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Data {
  a: String,
  b: String,
  E: i64,
  P: String,
  s: String,
  v: String,
}

impl<'de> Deserialize<'de> for Ticker {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let ticker = BinanceTicker::deserialize(deserializer)?.data;

    Ok(Ticker {
      ask_price: f32::from_str(&ticker.a).unwrap(),
      bid_price: f32::from_str(&ticker.b).unwrap(),
      ticker_name: ticker.s.to_lowercase(),
      change: f32::from_str(&ticker.P).unwrap(),
      volume: f32::from_str(&ticker.v).unwrap(),
      timestamp: ticker.E,
    })
  }
}
