use super::price::{BasePrice, Price};
use serde::Serialize;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize)]
pub struct Oracle {
  pub symbol: String,
  pub ask_best_exchange: String,
  pub ask_best_price: f32,
  pub ask_avg_price: f32,
  pub bid_best_exchange: String,
  pub bid_best_price: f32,
  pub bid_avg_price: f32,
  pub prices: HashMap<String, Price>,
}

#[derive(Clone, Debug, Serialize)]
pub struct OracleJson {
  pub symbol: String,
  pub ask_best_exchange: String,
  pub ask_best_price: f32,
  pub ask_avg_price: f32,
  pub bid_best_exchange: String,
  pub bid_best_price: f32,
  pub bid_avg_price: f32,
  pub prices: Vec<Price>,
}

impl Oracle {
  pub fn to_json(&self) -> OracleJson {
    OracleJson {
      symbol: self.symbol.clone(),
      ask_best_exchange: self.ask_best_exchange.clone(),
      ask_best_price: self.ask_best_price.clone(),
      ask_avg_price: self.ask_avg_price.clone(),
      bid_best_exchange: self.bid_best_exchange.clone(),
      bid_best_price: self.bid_best_price.clone(),
      bid_avg_price: self.bid_avg_price.clone(),
      prices: self.prices.clone().into_values().collect(),
    }
  }

  pub fn update_price(
    &mut self,
    BasePrice {
      exchange,
      ask_price,
      bid_price,
    }: BasePrice,
  ) -> Price {
    self.prices.insert(
      exchange.clone(),
      Price {
        exchange: exchange.clone(),
        ask_premium: 0.0,
        ask_price,
        bid_premium: 0.0,
        bid_price,
      },
    );

    if exchange.clone() == self.ask_best_exchange || ask_price > self.ask_best_price {
      let price = self
        .prices
        .clone()
        .into_values()
        .min_by(|x, y| x.ask_price.partial_cmp(&y.ask_price).unwrap_or(Equal))
        .unwrap();

      self.ask_best_exchange = price.exchange;
      self.ask_best_price = price.ask_price;
    };

    if exchange.clone() == self.bid_best_exchange || bid_price > self.bid_best_price {
      let price = self
        .prices
        .clone()
        .into_values()
        .max_by(|x, y| x.bid_price.partial_cmp(&y.bid_price).unwrap_or(Equal))
        .unwrap();

      self.bid_best_exchange = price.exchange;
      self.bid_best_price = price.bid_price;
    };

    self.ask_avg_price = 0.0;
    self.bid_avg_price = 0.0;
    self.prices.clone().into_values().for_each(|price| {
      let price = self.prices.get_mut(&price.exchange).unwrap();
      self.ask_avg_price += price.ask_price;
      self.bid_avg_price += price.bid_price;
      price.ask_premium = (price.ask_price / self.ask_best_price) - 1.0;
      price.bid_premium = (price.bid_price / self.bid_best_price) - 1.0;
    });

    let prices_size = self.prices.len() as f32;
    self.ask_avg_price = self.ask_avg_price / prices_size;
    self.bid_avg_price = self.bid_avg_price / prices_size;

    self.prices.get(&exchange).unwrap().clone()
  }
}
