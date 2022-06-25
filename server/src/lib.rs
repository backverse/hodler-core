mod cors;
mod get_cryptocurrencies;
mod get_currencies;
mod get_insights;
mod get_overviews;
mod health_check;

use axum::{extract::Query, middleware, routing::get, Router, Server};
use config::PORT;
use get_insights::Parameters;
use std::sync::{Arc, Mutex};

type Hodler = Arc<Mutex<hodler::Hodler>>;

pub struct HodlerServer {}

impl HodlerServer {
  pub async fn new(h: Hodler) {
    let health_check = get(health_check::handler);
    let get_cryptocurrencies = |h: Hodler| get(move || get_cryptocurrencies::handler(h));
    let get_currencies = |h: Hodler| get(move || get_currencies::handler(h));
    let get_overviews = |h: Hodler| get(move || get_overviews::handler(h));
    let get_insights =
      |h: Hodler| get(move |Query(query): Query<Parameters>| get_insights::handler(query, h));

    let router = Router::new()
      .route("/", health_check)
      .route("/cryptocurrencies", get_cryptocurrencies(h.clone()))
      .route("/currencies", get_currencies(h.clone()))
      .route("/overviews", get_overviews(h.clone()))
      .route("/insights", get_insights(h.clone()))
      .route_layer(middleware::from_fn(cors::handler));

    Server::bind(&format!("0.0.0.0:{PORT}").parse().unwrap())
      .serve(router.into_make_service())
      .await
      .unwrap();
  }
}
