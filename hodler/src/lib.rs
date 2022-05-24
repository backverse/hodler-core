pub mod models;

use self::models::price::{BasePrice, Price};
use self::models::ticker::MarketTicker;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Hodler {
  pub base_prices: HashMap<String, BasePrice>,
  pub prices: HashMap<String, HashMap<String, Price>>,
}

impl Hodler {
  pub fn new() -> Arc<Mutex<Self>> {
    let hodler = Self {
      base_prices: HashMap::new(),
      prices: HashMap::new(),
    };

    Arc::new(Mutex::new(hodler))
  }

  pub fn update_base_price(&mut self, market_ticker: &MarketTicker) {
    self.base_prices.insert(
      market_ticker.exchange.clone(),
      BasePrice {
        exchange: market_ticker.exchange.clone(),
        ask_price: market_ticker.ask_price,
        bid_price: market_ticker.bid_price,
        timestamp: market_ticker.timestamp,
      },
    );
  }

  pub fn update_price(&mut self, market_ticker: MarketTicker) {
    if market_ticker.is_base_ticker() {
      return self.update_base_price(&market_ticker);
    }

    let base_price = match self.base_prices.get(&market_ticker.exchange) {
      Some(base_price) => base_price,
      None => return,
    };

    let ask_price = market_ticker.ask_price / base_price.ask_price;
    let bid_price = market_ticker.bid_price / base_price.bid_price;

    match self.prices.get_mut(&market_ticker.symbol) {
      Some(price) => {
        price.insert(
          market_ticker.exchange.clone(),
          Price {
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
        self.prices.insert(
          market_ticker.symbol.clone(),
          HashMap::from([(
            market_ticker.exchange.clone(),
            Price {
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
