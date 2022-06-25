use axum::{http::StatusCode, response::IntoResponse, Json};
use hodler::{models::currency::Cryptocurrency, Hodler};
use std::sync::{Arc, Mutex};

pub async fn handler(hodler: Arc<Mutex<Hodler>>) -> impl IntoResponse {
  let cryptocurrencies = hodler
    .lock()
    .unwrap()
    .cryptocurrencies
    .clone()
    .into_values()
    .map(|exchanges| exchanges.clone().into_values().collect())
    .collect::<Vec<Vec<Cryptocurrency>>>();

  (StatusCode::OK, Json(cryptocurrencies))
}
