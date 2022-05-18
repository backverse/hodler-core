use serde::Deserialize;
use serde::Deserializer;
use std::str::FromStr;

#[derive(Debug)]
pub struct Ticker {
  pub ask_price: f32,
  pub bid_price: f32,
  pub symbol: String,
}

impl<'de> Deserialize<'de> for Ticker {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct BinanceTicker {
      data: Data,
    }

    #[derive(Deserialize)]
    struct Data {
      a: String,
      b: String,
      s: String,
    }

    let ticker = BinanceTicker::deserialize(deserializer)?;

    Ok(Ticker {
      ask_price: f32::from_str(&ticker.data.a).unwrap(),
      bid_price: f32::from_str(&ticker.data.b).unwrap(),
      symbol: ticker.data.s.to_lowercase(),
    })
  }
}
