use super::price::Price;
use serde::Serialize;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize)]
pub struct Oracle {
  pub symbol: String,
  pub ask_best_exchange: String,
  pub ask_best_symbol: String,
  pub ask_best_price: f32,
  pub ask_avg_price: f32,
  pub bid_best_exchange: String,
  pub bid_best_symbol: String,
  pub bid_best_price: f32,
  pub bid_best_original: f32,
  pub bid_avg_price: f32,
  pub prices: HashMap<String, Price>,
}

#[derive(Clone, Debug, Serialize)]
pub struct OracleJson {
  pub symbol: String,
  pub ask_best_exchange: String,
  pub ask_best_symbol: String,
  pub ask_best_price: f32,
  pub ask_avg_price: f32,
  pub bid_best_exchange: String,
  pub bid_best_symbol: String,
  pub bid_best_price: f32,
  pub bid_best_original: f32,
  pub bid_avg_price: f32,
  pub prices: Vec<Price>,
  pub icon_id: String,
}

impl Oracle {
  pub fn to_json(&self) -> OracleJson {
    OracleJson {
      symbol: self.symbol.clone(),
      ask_best_symbol: self.ask_best_symbol.clone(),
      ask_avg_price: self.ask_avg_price.clone(),
      ask_best_exchange: self.ask_best_exchange.clone(),
      ask_best_price: self.ask_best_price.clone(),
      bid_best_symbol: self.bid_best_symbol.clone(),
      bid_avg_price: self.bid_avg_price.clone(),
      bid_best_exchange: self.bid_best_exchange.clone(),
      bid_best_price: self.bid_best_price.clone(),
      bid_best_original: self.bid_best_original.clone(),
      prices: self.prices.clone().into_values().collect(),
      icon_id: match self.symbol.as_str() {
        "btc" => "1",
        "eth" => "1027",
        "dot" => "6636",
        "pow" => "2132",
        "ltc" => "2",
        "mana" => "1966",
        "near" => "6535",
        "zil" => "2469",
        "doge" => "74",
        "bnb" => "1839",
        "iost" => "2405",
        "sand" => "6210",
        "gala" => "7080",
        "sol" => "5426",
        "avax" => "5805",
        _ => "",
      }
      .to_string(),
    }
  }

  pub fn update_price(&mut self, price: Price) -> Price {
    let exchange = price.exchange.clone();
    let ask_price = price.ask_price.clone();
    let bid_price = price.bid_price.clone();

    self.prices.insert(exchange.clone(), price);

    if exchange.clone() == self.ask_best_exchange || ask_price < self.ask_best_price {
      let price = self
        .prices
        .clone()
        .into_values()
        .min_by(|x, y| x.ask_price.partial_cmp(&y.ask_price).unwrap_or(Equal))
        .unwrap();

      self.ask_best_exchange = price.exchange;
      self.ask_best_symbol = price.symbol;
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
      self.bid_best_symbol = price.symbol;
      self.bid_best_price = price.bid_price;
      self.bid_best_original = price.bid_original;
    };

    self.ask_avg_price = 0.0;
    self.bid_avg_price = 0.0;
    self.prices.clone().into_values().for_each(|price| {
      let price = self.prices.get_mut(&price.exchange).unwrap();
      self.ask_avg_price += price.ask_price;
      self.bid_avg_price += price.bid_price;
      price.arbitrage = (self.bid_best_price / price.ask_price) - 1.0;
      price.ask_premium = (price.ask_price / self.ask_best_price) - 1.0;
      price.bid_premium = (price.bid_price / self.bid_best_price) - 1.0;
    });

    let prices_size = self.prices.len() as f32;
    self.ask_avg_price = self.ask_avg_price / prices_size;
    self.bid_avg_price = self.bid_avg_price / prices_size;

    self.prices.get(&exchange).unwrap().clone()
  }
}
