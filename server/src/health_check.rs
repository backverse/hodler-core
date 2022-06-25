use axum::{http::StatusCode, response::IntoResponse, Json};
use std::collections::HashMap;

pub async fn handler() -> impl IntoResponse {
  (StatusCode::OK, Json(HashMap::from([("status", "OK")])))
}
