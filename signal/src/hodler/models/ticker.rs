use super::price::{BasePrice, BaseSymbol};

#[derive(Clone)]
pub struct MarketTicker {
  pub exchange: String,
  pub symbol: String,
  pub symbol_key: String,
  pub ask_price: f32,
  pub bid_price: f32,
}

impl MarketTicker {
  pub fn is_base_ticker(&self) -> bool {
    self.symbol_key.contains(BaseSymbol::BTC.value())
  }

  pub fn to_base_price(&self) -> BasePrice {
    BasePrice {
      exchange: self.exchange.clone(),
      ask_price: self.ask_price,
      bid_price: self.bid_price,
    }
  }
}
