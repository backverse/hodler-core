use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Signal {
  pub side: SignalSide,
  pub exchange: String,
  pub symbol: String,
  pub symbol_key: String,
  pub original_price: f32,
  pub price: f32,
}

#[derive(Clone, Debug, Serialize)]
pub enum SignalSide {
  Buy,
  Sell,
}

pub enum SignalThreshold {
  Arbitrage,
}

impl SignalThreshold {
  pub fn value(&self) -> f32 {
    match self {
      Self::Arbitrage => config::SIGNAL_THRESHOLD,
    }
  }
}
