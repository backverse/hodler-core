use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct BasePrice {
  pub exchange: String,
  pub ask_price: f32,
  pub bid_price: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct Price {
  pub exchange: String,
  pub symbol: String,
  pub arbitrage: f32,
  pub ask_original: f32,
  pub ask_premium: f32,
  pub ask_price: f32,
  pub bid_original: f32,
  pub bid_premium: f32,
  pub bid_price: f32,
}

#[derive(Debug)]
pub enum BaseSymbol {
  BTC,
}

impl BaseSymbol {
  pub fn value(&self) -> &str {
    match self {
      Self::BTC => "btc",
    }
  }
}
