pub mod models;

use self::models::oracle::Oracle;
use self::models::price::{BasePrice, Price};
use self::models::signal::SignalThreshold;
use self::models::signal::{Signal, SignalSide};
use self::models::ticker::MarketTicker;
use log::debug;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Hodler {
  pub bases: HashMap<String, BasePrice>,
  pub oracles: HashMap<String, Oracle>,
}

impl Hodler {
  pub fn new() -> Arc<Mutex<Self>> {
    let hodler = Self {
      bases: HashMap::new(),
      oracles: HashMap::new(),
    };

    Arc::new(Mutex::new(hodler))
  }

  pub fn update_base_price(&mut self, market_ticker: &MarketTicker) {
    self.bases.insert(
      market_ticker.exchange.clone(),
      market_ticker.to_base_price(),
    );
  }

  pub async fn update_market(&mut self, market_ticker: MarketTicker) {
    if market_ticker.is_base_ticker() {
      return self.update_base_price(&market_ticker);
    }

    let base_price = match self.bases.get(&market_ticker.exchange) {
      Some(base_price) => base_price,
      None => return,
    };

    let ask_price = market_ticker.ask_price / base_price.ask_price;
    let bid_price = market_ticker.bid_price / base_price.bid_price;

    match self.oracles.get_mut(&market_ticker.symbol_key) {
      None => {
        self.oracles.insert(
          market_ticker.symbol_key.clone(),
          Oracle {
            symbol: market_ticker.symbol_key,
            ask_best_symbol: market_ticker.symbol.clone(),
            ask_best_exchange: market_ticker.exchange.clone(),
            ask_best_price: ask_price,
            ask_avg_price: ask_price,
            bid_best_symbol: market_ticker.symbol.clone(),
            bid_best_exchange: market_ticker.exchange.clone(),
            bid_best_price: bid_price,
            bid_best_original: market_ticker.bid_price,
            bid_avg_price: bid_price,
            prices: HashMap::from([(
              market_ticker.exchange.clone(),
              Price {
                exchange: market_ticker.exchange,
                symbol: market_ticker.symbol,
                arbitrage: 0.0,
                ask_original: market_ticker.ask_price,
                ask_premium: 0.0,
                ask_price,
                bid_original: market_ticker.bid_price,
                bid_premium: 0.0,
                bid_price,
              },
            )]),
          },
        );
      }
      Some(oracle) => {
        let price = Price {
          exchange: market_ticker.exchange.clone(),
          symbol: market_ticker.symbol.clone(),
          arbitrage: 0.0,
          ask_original: market_ticker.ask_price,
          ask_premium: 0.0,
          ask_price,
          bid_original: market_ticker.bid_price,
          bid_premium: 0.0,
          bid_price,
        };
        let price = oracle.update_price(price);
        if price.arbitrage < SignalThreshold::Arbitrage.value() {
          return;
        };

        let bid_best_exchange = oracle.bid_best_exchange.clone();
        let bid_best_price = oracle.bid_best_price.clone();
        let bid_best_original = oracle.bid_best_original.clone();

        self
          .publish(Signal {
            side: SignalSide::Buy,
            exchange: market_ticker.exchange.clone(),
            symbol: market_ticker.symbol.clone(),
            symbol_key: market_ticker.symbol_key.clone(),
            original_price: market_ticker.ask_price,
            price: price.ask_price,
          })
          .await;

        self
          .publish(Signal {
            side: SignalSide::Sell,
            exchange: bid_best_exchange,
            symbol: market_ticker.symbol.clone(),
            symbol_key: market_ticker.symbol_key.clone(),
            original_price: bid_best_original,
            price: bid_best_price,
          })
          .await;
      }
    };
  }

  pub async fn publish(&self, signal: Signal) {
    debug!(target: "hodler", "{signal:?}");
  }
}
