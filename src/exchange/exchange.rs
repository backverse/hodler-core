#[derive(Clone)]
pub enum Exchange {
  BinanceEx,
  BitkubEx,
}

impl Exchange {
  pub fn get_name(&self) -> String {
    match self {
      Self::BinanceEx => "binance",
      Self::BitkubEx => "bitkub",
    }
    .to_string()
  }

  pub fn get_key(&self, symbol: String) -> String {
    match self {
      Self::BinanceEx => match symbol.replace("usdt", "").as_str() {
        "powr" => "pow",
        symbol => &symbol,
      }
      .to_string(),
      Self::BitkubEx => symbol.replace("market.ticker.thb_", ""),
    }
  }
}
