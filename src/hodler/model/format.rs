#[derive(Debug)]
pub enum KeyFormat {
  MarketTicker,
  Signal,
}

impl KeyFormat {
  pub fn of(&self, symbol_key: String, exchange: String) -> String {
    match self {
      Self::MarketTicker => format!("market_tickers:{}:{}", symbol_key, exchange),
      Self::Signal => format!("signals:{}:{}", symbol_key, exchange),
    }
  }
}
