use super::format::KeyFormat;
use serde::Serialize;
use serde_json::to_string;

#[derive(Clone, Serialize)]
pub struct Signal {
  pub exchange: String,
  pub symbol: String,
  pub symbol_key: String,
  pub premium: f32,
  pub price: f32,
  pub side: SignalSide,
}

impl Signal {
  pub fn get_key(&self) -> String {
    KeyFormat::Signal.of(self.symbol_key.clone(), self.exchange.clone())
  }

  pub fn get_value(&self) -> String {
    to_string(self).unwrap()
  }
}

#[derive(Clone, Serialize)]
pub enum SignalSide {
  Buy,
  Sell,
}

pub enum SignalThreshold {
  Ask,
  Bid,
}

impl SignalThreshold {
  pub fn value(&self) -> f32 {
    match self {
      Self::Ask => 0.0,
      Self::Bid => 0.0,
    }
  }
}
