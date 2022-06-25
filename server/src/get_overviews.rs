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
      let mut exchanges = exchanges.clone().into_values();
      let exchange = exchanges.next().unwrap();
      let mut symbol = exchange.symbol;
      let mut sum_volume = exchange.volume;
      let mut sum_percent_change = exchange.percent_change;
      let mut sum_ask_price = exchange.ask_price;
      let mut sum_bid_price = exchange.bid_price;
      let mut n: f32 = 1.0;
      let mut best_ask_exchange = exchange.exchange.clone();
      let mut best_ask_price = exchange.ask_price;
      let mut best_ask_ticker_name = exchange.ticker_name.clone();
      let mut best_bid_exchange = exchange.exchange;
      let mut best_bid_price = exchange.ask_price;
      let mut best_bid_ticker_name = exchange.ticker_name;
      let mut best_arbitrage = 1.0 - exchange.bid_price / exchange.ask_price;

      exchanges.for_each(|cryptocurrency| {
        symbol = cryptocurrency.clone().symbol.clone();
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

        let arbitrage_rate = 1.0 - cryptocurrency.bid_price / cryptocurrency.ask_price;
        if arbitrage_rate > best_arbitrage {
          best_arbitrage = arbitrage_rate;
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
        volume: sum_volume * sum_ask_price / n,
        percent_change: sum_percent_change / n,
        icon_id: match symbol.as_str() {
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
      }
    })
    .collect::<Vec<Overview>>();

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
  pub icon_id: String,
}
