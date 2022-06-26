use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Overview {
  pub symbol: String,
  pub volume: f32,
  pub percent_change: f32,
  pub average_ask_price: f32,
  pub average_bid_price: f32,
  pub best_ask_exchange: String,
  pub best_ask_price: f32,
  pub best_ask_ticker_name: String,
  pub best_bid_exchange: String,
  pub best_bid_price: f32,
  pub best_bid_ticker_name: String,
  pub best_arbitrage: f32,
  pub icon_id: String,
}
