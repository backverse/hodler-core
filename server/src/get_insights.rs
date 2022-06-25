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
  let cryptocurrencies = exchanges.clone().into_values().enumerate();
  let mut exchanges = exchanges.into_values();
  let exchange = exchanges.next().unwrap();
  let symbol = exchange.symbol;
  let mut sum_volume = exchange.volume;
  let mut sum_percent_change = exchange.percent_change;
  let mut sum_ask_price = exchange.ask_price;
  let mut sum_bid_price = exchange.bid_price;
  let mut n: f32 = 1.0;
  let mut best_ask_exchange = exchange.exchange.clone();
  let mut best_ask_price = exchange.ask_price;
  let mut best_ask_ticker_name = exchange.ticker_name.clone();
  let mut best_bid_exchange = exchange.exchange;
  let mut best_bid_price = exchange.bid_price;
  let mut best_bid_ticker_name = exchange.ticker_name;
  let mut best_arbitrage = 0.0;
  let mut best_ask_premium = 0.0;
  let mut best_bid_premium = 0.0;

  exchanges.for_each(|cryptocurrency| {
    sum_volume += cryptocurrency.volume.clone();
    sum_percent_change += cryptocurrency.percent_change.clone();
    sum_ask_price += cryptocurrency.ask_price.clone();
    sum_bid_price += cryptocurrency.bid_price.clone();
    n += 1.0;

    if cryptocurrency.ask_price.clone() < best_ask_price {
      best_ask_exchange = cryptocurrency.exchange.clone();
      best_ask_price = cryptocurrency.ask_price.clone();
      best_ask_ticker_name = cryptocurrency.ticker_name.clone();
    }

    if cryptocurrency.bid_price.clone() > best_bid_price {
      best_bid_exchange = cryptocurrency.exchange.clone();
      best_bid_price = cryptocurrency.bid_price.clone();
      best_bid_ticker_name = cryptocurrency.ticker_name.clone();
    }
  });

  cryptocurrencies.for_each(|(i, cryptocurrency)| {
    let arbitrage = Arbitrage {
      exchange: cryptocurrency.exchange.clone(),
      best_routes: best_bid_exchange.clone(),
      rate: (1.0 - cryptocurrency.bid_price / cryptocurrency.ask_price) * 100.0,
    };

    let premium = Premium {
      exchange: cryptocurrency.exchange,
      ask_premium: (1.0 - cryptocurrency.ask_price / best_ask_price) * 100.0,
      ask_price: cryptocurrency.ask_price,
      bid_premium: (1.0 - cryptocurrency.bid_price / best_bid_price) * 100.0,
      bid_price: cryptocurrency.bid_price,
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
    symbol_id: match symbol.as_str() {
      "btc" => "1",
      "eth" => "1027",
      "dot" => "6636",
      "powr" => "2132",
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
  pub exchange: String,
  pub best_routes: String,
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
  pub symbol_id: String,
}
