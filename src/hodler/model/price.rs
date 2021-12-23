#[derive(Clone, Debug)]
pub struct BasePrice {
  pub ask_price: f32,
  pub bid_price: f32,
}

#[derive(Clone, Debug)]
pub struct Price {
  pub ask_premium: f32,
  pub ask_price: f32,
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
