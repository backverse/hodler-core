use axum::{http::StatusCode, response::IntoResponse, Json};
use hodler::Hodler;
use serde::Serialize;
use std::sync::{Arc, Mutex};

pub async fn handler(hodler: Arc<Mutex<Hodler>>) -> impl IntoResponse {
  let mut currencies = hodler
    .lock()
    .unwrap()
    .currencies
    .clone()
    .into_values()
    .map(|currency| Currency {
      exchange: currency.exchange.clone(),
      ask_price: currency.ask_price,
      bid_price: currency.bid_price,
      code: match currency.exchange.as_str() {
        "bitkub" => "THB",
        _ => "USD",
      }
      .to_string(),
      symbol: match currency.exchange.as_str() {
        "bitkub" => "฿",
        _ => "attach_money",
      }
      .to_string(),
      fraction_digits: 2,
      updated_at: currency.timestamp,
    })
    .collect::<Vec<Currency>>();

  currencies.insert(
    0,
    Currency {
      exchange: "hodler".to_string(),
      ask_price: 1.0,
      bid_price: 1.0,
      code: "BTC".to_string(),
      symbol: "currency_bitcoin".to_string(),
      fraction_digits: 8,
      updated_at: 0,
    },
  );

  (StatusCode::OK, Json(currencies))
}

#[derive(Serialize)]
struct Currency {
  pub exchange: String,
  pub ask_price: f32,
  pub bid_price: f32,
  pub code: String,
  pub symbol: String,
  pub fraction_digits: u8,
  pub updated_at: i64,
}
