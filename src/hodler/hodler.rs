use super::model::oracle::OraclePrice;
use super::model::price::{BasePrice, Price};
use super::model::signal::SignalThreshold;
use super::model::signal::{Signal, SignalSide};
use super::model::ticker::MarketTicker;
use log::{debug, error, info};
use mini_redis::client::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Clone)]
pub struct Hodler {
  base: HashMap<String, BasePrice>,
  market: HashMap<String, Price>,
  oracle: HashMap<String, OraclePrice>,
  redis: Arc<Mutex<Client>>,
}

impl Hodler {
  pub fn new(redis: Client) -> Arc<Mutex<Self>> {
    let hodler = Self {
      base: HashMap::new(),
      market: HashMap::new(),
      oracle: HashMap::new(),
      redis: Arc::new(Mutex::new(redis)),
    };

    Arc::new(Mutex::new(hodler))
  }

  pub fn update_base_price(&mut self, market_ticker: &MarketTicker) {
    self.base.insert(
      market_ticker.exchange.clone(),
      market_ticker.to_base_price(),
    );
  }

  pub async fn update_market(&mut self, market_ticker: MarketTicker) {
    debug!("{:?}", self.base);
    debug!("{:?}", self.market);
    debug!("{:?}", self.oracle);

    if market_ticker.is_base_ticker() {
      return self.update_base_price(&market_ticker);
    }

    let base_price = self.base.get(&market_ticker.exchange);

    if let None = base_price {
      return;
    }

    let base_price = base_price.unwrap();
    let ask_price = market_ticker.ask_price / base_price.ask_price;
    let bid_price = market_ticker.bid_price / base_price.bid_price;

    match self.oracle.get_mut(&market_ticker.symbol_key) {
      None => {
        self.oracle.insert(
          market_ticker.symbol_key,
          OraclePrice {
            ask_best: ask_price,
            ask_price,
            bid_best: bid_price,
            bid_price,
            prices: HashMap::from([(
              market_ticker.exchange,
              BasePrice {
                ask_price,
                bid_price,
              },
            )]),
          },
        );
      }
      Some(oracle_price) => {
        let ask_premium = (ask_price / oracle_price.ask_best) - 1.0;
        let bid_premium = (bid_price / oracle_price.bid_best) - 1.0;

        oracle_price.update_best_price(market_ticker.exchange.clone(), ask_price, bid_price);

        if ask_premium > SignalThreshold::Ask.value() {
          let signal = market_ticker.to_signal(SignalSide::Buy, ask_premium);
          self.publish(signal).await;
        };

        if bid_premium > SignalThreshold::Bid.value() {
          let signal = market_ticker.to_signal(SignalSide::Sell, bid_premium);
          self.publish(signal).await;
        };

        self.market.insert(
          market_ticker.get_key(),
          Price {
            ask_premium,
            ask_price,
            bid_premium,
            bid_price,
          },
        );
      }
    };
  }

  pub async fn publish(&self, signal: Signal) {
    let key = &signal.get_key();
    let value = signal.get_value();

    let mut redis = match self.redis.lock() {
      Ok(redis) => redis,
      Err(err) => return error!("unable to unwrap redis client: {:?}", err),
    };

    match timeout(
      Duration::from_millis(10),
      redis.publish(key, value.clone().into()),
    )
    .await
    {
      Ok(_) => info!("{}\n{:?}\n", key, value),
      Err(err) => error!("{:?}", err),
    };
  }
}
