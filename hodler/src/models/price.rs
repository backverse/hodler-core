use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct BasePrice {
  pub exchange: String,
  pub ask_price: f32,
  pub bid_price: f32,
  pub timestamp: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Price {
  pub exchange: String,
  pub symbol: String,
  pub ticker_name: String,
  pub ask_original: f32,
  pub ask_price: f32,
  pub bid_original: f32,
  pub bid_price: f32,
  pub volume: f32,
  pub percent_change: f32,
  pub timestamp: i64,
}
