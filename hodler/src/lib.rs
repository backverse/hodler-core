pub mod models;

use self::models::currency::{Cryptocurrency, Currency};
use self::models::ticker::MarketTicker;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Hodler {
  pub currencies: HashMap<String, Currency>,
  pub cryptocurrencies: HashMap<String, HashMap<String, Cryptocurrency>>,
}

impl Hodler {
  pub fn new() -> Arc<Mutex<Self>> {
    let hodler = Self {
      currencies: HashMap::new(),
      cryptocurrencies: HashMap::new(),
    };

    Arc::new(Mutex::new(hodler))
  }

  pub fn upsert_currency(&mut self, market_ticker: &MarketTicker) {
    self.currencies.insert(
      market_ticker.exchange.clone(),
      Currency {
        exchange: market_ticker.exchange.clone(),
        ask_price: market_ticker.ask_price,
        bid_price: market_ticker.bid_price,
        timestamp: market_ticker.timestamp,
      },
    );
  }

  pub fn upsert_cryptocurrency(&mut self, market_ticker: MarketTicker) {
    if market_ticker.is_currency_ticker() {
      return self.upsert_currency(&market_ticker);
    }

    let currency = match self.currencies.get(&market_ticker.exchange) {
      Some(currency) => currency,
      None => return,
    };

    let ask_price = market_ticker.ask_price / currency.ask_price;
    let bid_price = market_ticker.bid_price / currency.bid_price;

    match self.cryptocurrencies.get_mut(&market_ticker.symbol) {
      Some(cryptocurrency) => {
        cryptocurrency.insert(
          market_ticker.exchange.clone(),
          Cryptocurrency {
            exchange: market_ticker.exchange,
            symbol: market_ticker.symbol,
            ticker_name: market_ticker.ticker_name,
            ask_original: market_ticker.ask_price,
            ask_price,
            bid_original: market_ticker.bid_price,
            bid_price,
            volume: market_ticker.volume,
            percent_change: market_ticker.percent_change,
            timestamp: market_ticker.timestamp,
          },
        );
      }
      None => {
        self.cryptocurrencies.insert(
          market_ticker.symbol.clone(),
          HashMap::from([(
            market_ticker.exchange.clone(),
            Cryptocurrency {
              exchange: market_ticker.exchange,
              symbol: market_ticker.symbol,
              ticker_name: market_ticker.ticker_name,
              ask_original: market_ticker.ask_price,
              ask_price,
              bid_original: market_ticker.bid_price,
              bid_price,
              volume: market_ticker.volume,
              percent_change: market_ticker.percent_change,
              timestamp: market_ticker.timestamp,
            },
          )]),
        );
      }
    };
  }
}
