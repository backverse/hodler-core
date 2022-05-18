use serde::Deserialize;
use serde::Deserializer;

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
      market: String,
    }

    #[derive(Deserialize)]
    struct Data {
      ask: f32,
      bid: f32,
    }

    let ticker = BinanceTicker::deserialize(deserializer)?;

    Ok(Ticker {
      ask_price: ticker.data.ask,
      bid_price: ticker.data.bid,
      symbol: ticker.market.to_lowercase().replace("/usd", ""),
    })
  }
}
