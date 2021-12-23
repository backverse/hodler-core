use super::price::BasePrice;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct OraclePrice {
  pub ask_best: f32,
  pub ask_price: f32,
  pub bid_best: f32,
  pub bid_price: f32,
  pub prices: HashMap<String, BasePrice>,
}

impl OraclePrice {
  pub fn update_best_price(&mut self, exchange: String, ask_price: f32, bid_price: f32) {
    let is_best_ask = ask_price < self.ask_best;
    let is_best_bid = bid_price > self.bid_best;

    self.prices.insert(
      exchange,
      BasePrice {
        ask_price,
        bid_price,
      },
    );

    if is_best_ask {
      self.ask_best = ask_price
    }

    if is_best_bid {
      self.bid_best = bid_price
    }

    let len = self.prices.len() as f32;
    self.ask_price = self
      .prices
      .clone()
      .into_values()
      .map(|base_price| base_price.ask_price)
      .sum::<f32>()
      / len;
    self.bid_price = self
      .prices
      .clone()
      .into_values()
      .map(|base_price| base_price.bid_price)
      .sum::<f32>()
      / len;
  }
}
