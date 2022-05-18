use config;

#[derive(Clone)]
pub enum Exchange {
  BinanceEx,
  BitkubEx,
  FtxEx,
}

impl Exchange {
  pub fn get_name(&self) -> String {
    match self {
      Self::BinanceEx => "binance",
      Self::BitkubEx => "bitkub",
      Self::FtxEx => "ftx",
    }
    .to_string()
  }

  pub fn get_ticker_url(&self) -> String {
    let ticker_url_format = match self {
      Self::BinanceEx => "wss://stream.binance.com:9443/stream?streams={}",
      Self::BitkubEx => "wss://api.bitkub.com/websocket-api/{}",
      Self::FtxEx => "wss://ftx.com/ws/",
    };

    let tickers = self.get_tickers().join(self.get_ticker_sep());

    ticker_url_format.replace("{}", &tickers)
  }

  pub fn get_tickers(&self) -> Vec<String> {
    let ticker_format = match self {
      Self::BinanceEx => "{}usdt@ticker",
      Self::BitkubEx => "market.ticker.thb_{}",
      Self::FtxEx => "{\"op\": \"subscribe\", \"channel\": \"ticker\", \"market\": \"{}/USD\"}",
    };

    config::SYMBOLS
      .split(",")
      .map(|symbol| {
        ticker_format
          .clone()
          .replace("{}", &self.get_ticker(symbol))
      })
      .collect::<Vec<String>>()
  }

  fn get_ticker(&self, symbol: &str) -> String {
    match self {
      Self::BinanceEx => match symbol.clone() {
        "pow" => "powr",
        symbol => &symbol,
      },
      Self::BitkubEx => match symbol.clone() {
        "powr" => "pow",
        symbol => &symbol,
      },
      Self::FtxEx => symbol,
    }
    .to_string()
  }

  fn get_ticker_sep(&self) -> &str {
    match self {
      Self::BinanceEx => "/",
      Self::BitkubEx => ",",
      Self::FtxEx => "",
    }
  }

  pub fn get_key(&self, symbol: String) -> String {
    match self {
      Self::BinanceEx => match symbol.replace("usdt", "").as_str() {
        "powr" => "pow",
        symbol => &symbol,
      }
      .to_string(),
      Self::BitkubEx => symbol.replace("market.ticker.thb_", ""),
      Self::FtxEx => symbol.replace("-USD", ""),
    }
  }
}
