#[derive(Clone)]
pub struct MarketTicker {
  pub exchange: String,
  pub symbol: String,
  pub ticker_name: String,
  pub ask_price: f32,
  pub bid_price: f32,
  pub volume: f32,
  pub percent_change: f32,
  pub timestamp: i64,
}

impl MarketTicker {
  pub fn is_currency_ticker(&self) -> bool {
    self.symbol == config::CURRENCY_SYMBOL
  }
}
