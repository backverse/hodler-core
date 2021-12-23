use super::{
  format::KeyFormat,
  price::{BasePrice, BaseSymbol},
  signal::{Signal, SignalSide},
};

#[derive(Clone)]
pub struct MarketTicker {
  pub exchange: String,
  pub symbol: String,
  pub symbol_key: String,
  pub ask_price: f32,
  pub bid_price: f32,
}

impl MarketTicker {
  pub fn get_key(&self) -> String {
    KeyFormat::MarketTicker.of(self.symbol_key.clone(), self.exchange.clone())
  }

  pub fn is_base_ticker(&self) -> bool {
    self.symbol_key.contains(BaseSymbol::BTC.value())
  }

  pub fn to_base_price(&self) -> BasePrice {
    BasePrice {
      ask_price: self.ask_price,
      bid_price: self.bid_price,
    }
  }

  pub fn to_signal(&self, side: SignalSide, premium: f32) -> Signal {
    Signal {
      exchange: self.exchange.clone(),
      symbol: self.symbol.clone(),
      symbol_key: self.symbol_key.clone(),
      side: side.clone(),
      premium,
      price: match side {
        SignalSide::Buy => self.ask_price,
        SignalSide::Sell => self.bid_price,
      },
    }
  }
}
