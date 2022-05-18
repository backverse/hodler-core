use config;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Signal {
  pub exchange: String,
  pub symbol: String,
  pub symbol_key: String,
  pub premium: f32,
  pub price: f32,
  pub side: SignalSide,
}

#[derive(Clone, Debug, Serialize)]
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
      Self::Ask => config::SIGNAL_THRESHOLD_ASK,
      Self::Bid => config::SIGNAL_THRESHOLD_BID,
    }
  }
}
