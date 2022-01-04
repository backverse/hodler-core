use crate::config;

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

  pub fn get_ticker_url(&self) -> String {
    let ticker_url_format = match self {
      Self::BinanceEx => "wss://stream.binance.com:9443/stream?streams={}",
      Self::BitkubEx => "wss://api.bitkub.com/websocket-api/{}",
    };

    let ticker_format = match self {
      Self::BinanceEx => "{}usdt@ticker",
      Self::BitkubEx => "market.ticker.thb_{}",
    };

    let ticker_sep = match self {
      Self::BinanceEx => "/",
      Self::BitkubEx => ",",
    };

    let tickers = config::SYMBOLS
      .split(",")
      .map(|symbol| ticker_format.clone().replace("{}", symbol))
      .collect::<Vec<String>>()
      .join(ticker_sep);

    ticker_url_format.replace("{}", &tickers)
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
