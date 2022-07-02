use axum::{http::StatusCode, response::IntoResponse, Json};
use hodler::Hodler;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct Parameters {
  symbol: String,
}

pub async fn handler(query: Parameters, hodler: Arc<Mutex<Hodler>>) -> impl IntoResponse {
  let exchanges = match hodler
    .lock()
    .unwrap()
    .cryptocurrencies
    .clone()
    .get(&query.symbol)
  {
    Some(e) => e.clone(),
    None => {
      return (StatusCode::NOT_FOUND, Json(None));
    }
  };

  let mut arbitrages = Vec::<Arbitrage>::new();
  let mut premiums = Vec::<Premium>::new();
  let cryptocurrencies_with_indexes = exchanges.clone().into_values().enumerate();
  let mut cryptocurrencies = exchanges.into_values();
  let cryptocurrency = cryptocurrencies.next().unwrap();
  let symbol = cryptocurrency.symbol;
  let mut sum_volume = cryptocurrency.volume;
  let mut sum_percent_change = cryptocurrency.percent_change;
  let mut sum_ask_price = cryptocurrency.ask_price;
  let mut sum_bid_price = cryptocurrency.bid_price;
  let mut n: f32 = 1.0;
  let mut best_ask_exchange = cryptocurrency.exchange.clone();
  let mut best_ask_price = cryptocurrency.ask_price;
  let mut best_ask_ticker_name = cryptocurrency.ticker_name.clone();
  let mut best_bid_exchange = cryptocurrency.exchange;
  let mut best_bid_price = cryptocurrency.bid_price;
  let mut best_bid_ticker_name = cryptocurrency.ticker_name;
  let mut best_arbitrage = 0.0;
  let mut best_ask_premium = 0.0;
  let mut best_bid_premium = 0.0;

  cryptocurrencies.for_each(|c| {
    sum_volume += c.volume.clone();
    sum_percent_change += c.percent_change.clone();
    sum_ask_price += c.ask_price.clone();
    sum_bid_price += c.bid_price.clone();
    n += 1.0;

    if c.ask_price.clone() < best_ask_price {
      best_ask_exchange = c.exchange.clone();
      best_ask_price = c.ask_price.clone();
      best_ask_ticker_name = c.ticker_name.clone();
    }

    if c.bid_price.clone() > best_bid_price {
      best_bid_exchange = c.exchange;
      best_bid_price = c.bid_price;
      best_bid_ticker_name = c.ticker_name;
    }
  });

  cryptocurrencies_with_indexes.for_each(|(i, c)| {
    let arbitrage = Arbitrage {
      buy_low_exchange: c.exchange.clone(),
      buy_low_price: c.ask_price.clone(),
      sell_high_exchange: best_bid_exchange.clone(),
      sell_high_price: best_bid_price.clone(),
      rate: (best_bid_price / c.ask_price - 1.0) * 100.0,
    };

    let premium = Premium {
      exchange: c.exchange,
      ask_premium: (c.ask_price / best_ask_price - 1.0) * 100.0,
      ask_price: c.ask_price,
      bid_premium: (c.bid_price / best_bid_price - 1.0) * 100.0,
      bid_price: c.bid_price,
    };

    if i == 0 {
      best_arbitrage = arbitrage.rate;
      best_ask_premium = premium.ask_premium;
      best_bid_premium = premium.bid_premium;
      arbitrages.push(arbitrage);
      premiums.push(premium);
      return;
    }

    if arbitrage.rate > best_arbitrage {
      best_arbitrage = arbitrage.rate;
    }

    if premium.ask_premium < best_ask_premium {
      best_ask_premium = premium.ask_premium
    }

    if premium.bid_premium > best_arbitrage {
      best_bid_premium = premium.bid_premium;
    }

    arbitrages.push(arbitrage);
    premiums.push(premium);
  });

  let summary = Summary {
    symbol: symbol.clone(),
    average_ask_price: sum_ask_price / n,
    average_bid_price: sum_bid_price / n,
    best_ask_exchange,
    best_ask_price,
    best_ask_ticker_name,
    best_bid_exchange,
    best_bid_price,
    best_bid_ticker_name,
    volume: sum_volume * sum_ask_price / n,
    percent_change: sum_percent_change / n,
    best_arbitrage,
    best_ask_premium,
    best_bid_premium,
    icon: format!(
      "https://cdn.bitkubnow.com/coins/icon/{}.png",
      symbol.replace("powr", "pow").to_uppercase()
    ),
  };

  (
    StatusCode::OK,
    Json(Some(Insight {
      arbitrages,
      premiums,
      summary,
    })),
  )
}

#[derive(Serialize)]
pub struct Insight {
  pub arbitrages: Vec<Arbitrage>,
  pub premiums: Vec<Premium>,
  pub summary: Summary,
}

#[derive(Serialize)]
pub struct Arbitrage {
  pub buy_low_exchange: String,
  pub buy_low_price: f32,
  pub sell_high_exchange: String,
  pub sell_high_price: f32,
  pub rate: f32,
}

#[derive(Serialize)]
pub struct Premium {
  pub exchange: String,
  pub ask_premium: f32,
  pub ask_price: f32,
  pub bid_premium: f32,
  pub bid_price: f32,
}

#[derive(Serialize)]
pub struct Summary {
  pub symbol: String,
  pub volume: f32,
  pub percent_change: f32,
  pub average_ask_price: f32,
  pub average_bid_price: f32,
  pub best_ask_exchange: String,
  pub best_ask_price: f32,
  pub best_ask_ticker_name: String,
  pub best_bid_exchange: String,
  pub best_bid_price: f32,
  pub best_bid_ticker_name: String,
  pub best_arbitrage: f32,
  pub best_ask_premium: f32,
  pub best_bid_premium: f32,
  pub icon: String,
}
