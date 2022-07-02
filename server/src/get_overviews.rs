use axum::{http::StatusCode, response::IntoResponse, Json};
use hodler::Hodler;
use serde::Serialize;
use std::sync::{Arc, Mutex};

pub async fn handler(hodler: Arc<Mutex<Hodler>>) -> impl IntoResponse {
  let mut overviews = hodler
    .lock()
    .unwrap()
    .cryptocurrencies
    .clone()
    .into_values()
    .map(|exchanges| {
      let cryptocurrencies_with_indexes = exchanges.clone().into_values().enumerate();
      let mut cryptocurrencies = exchanges.clone().into_values();
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
      let mut best_bid_price = cryptocurrency.ask_price;
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
          best_bid_exchange = c.exchange.clone();
          best_bid_price = c.bid_price.clone();
          best_bid_ticker_name = c.ticker_name.clone();
        }
      });

      cryptocurrencies_with_indexes.for_each(|(i, c)| {
        let arbitrage_rate = best_bid_price / c.ask_price - 1.0;
        let ask_premium = c.ask_price / best_ask_price - 1.0;
        let bid_premium = c.bid_price / best_bid_price - 1.0;

        if i == 0 {
          best_arbitrage = arbitrage_rate;
          best_ask_premium = ask_premium;
          best_bid_premium = bid_premium;
          return;
        }

        if arbitrage_rate > best_arbitrage {
          best_arbitrage = arbitrage_rate;
        }

        if ask_premium < best_ask_premium {
          best_ask_premium = ask_premium
        }

        if bid_premium > best_arbitrage {
          best_bid_premium = bid_premium;
        }
      });

      Overview {
        symbol: symbol.clone(),
        average_ask_price: sum_ask_price / n,
        average_bid_price: sum_bid_price / n,
        best_ask_exchange,
        best_ask_price,
        best_ask_ticker_name,
        best_bid_exchange,
        best_bid_price,
        best_bid_ticker_name,
        best_arbitrage: best_arbitrage * 100.0,
        best_ask_premium: best_ask_premium * 100.0,
        best_bid_premium: best_bid_premium * 100.0,
        volume: sum_volume * sum_ask_price / n,
        percent_change: sum_percent_change / n,
        icon: format!(
          "https://cdn.bitkubnow.com/coins/icon/{}.png",
          symbol.replace("powr", "pow").to_uppercase()
        ),
      }
    })
    .collect::<Vec<Overview>>();

  overviews.sort_by(|a, b| b.best_arbitrage.partial_cmp(&a.best_arbitrage).unwrap());
  overviews.sort_by(|a, b| b.volume.partial_cmp(&a.volume).unwrap());

  (StatusCode::OK, Json(overviews))
}

#[derive(Serialize)]
pub struct Overview {
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
