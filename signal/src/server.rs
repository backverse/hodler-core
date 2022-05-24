use super::Hodler;
use hodler::models::price::{BasePrice, Price};
use hyper::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use log::{error, info};
use serde::Serialize;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive()]
pub struct HodlerServer {
  address: SocketAddr,
  hodler: Arc<Mutex<Hodler>>,
}

impl HodlerServer {
  pub fn new(hodler: Arc<Mutex<Hodler>>) -> HodlerServer {
    HodlerServer {
      address: SocketAddr::new("0.0.0.0".parse().unwrap(), config::PORT),
      hodler,
    }
  }

  pub async fn serve(&self) {
    let service = make_service_fn(move |_| {
      let hodler = self.hodler.clone();

      let svc_fn = service_fn(move |req| {
        let hodler = hodler.clone();

        async move {
          let hodler = hodler.lock().unwrap();
          let raw = match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
              json!({"status": "OK"})
            }
            (&Method::GET, "/base-prices") => {
              let base_prices = hodler
                .base_prices
                .clone()
                .into_values()
                .collect::<Vec<BasePrice>>();

              json!(base_prices)
            }
            (&Method::GET, "/prices") => {
              let prices = hodler
                .prices
                .clone()
                .into_values()
                .map(|exchanges| exchanges.clone().into_values().collect())
                .collect::<Vec<Vec<Price>>>();

              json!(prices)
            }
            (&Method::GET, "/overview") => {
              #[derive(Clone, Debug, Serialize)]
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
                pub icon_id: String,
              }

              let mut overview = hodler
                .prices
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

                  exchanges.for_each(|price| {
                    symbol = price.clone().symbol.clone();
                    sum_volume += price.volume.clone();
                    sum_percent_change += price.percent_change.clone();
                    sum_ask_price += price.ask_price.clone();
                    sum_bid_price += price.bid_price.clone();
                    n += 1.0;

                    if price.ask_price.clone() < best_ask_price {
                      best_ask_exchange = price.exchange.clone();
                      best_ask_price = price.ask_price.clone();
                      best_ask_ticker_name = price.ticker_name.clone();
                    }

                    if price.bid_price.clone() > best_bid_price {
                      best_bid_exchange = price.exchange.clone();
                      best_bid_price = price.bid_price.clone();
                      best_bid_ticker_name = price.ticker_name.clone();
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
                    volume: sum_volume / n * sum_ask_price / n,
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

              overview.sort_by(|a, b| b.volume.partial_cmp(&a.volume).unwrap());

              json!(overview)
            }
            (_, _) => {
              let mut response = Response::new(Body::empty());

              *response.status_mut() = StatusCode::NOT_FOUND;

              return Ok::<_, Error>(response);
            }
          };

          let body = Body::from(raw.to_string());
          let mut response = Response::builder().body(body).unwrap();
          let headers = response.headers_mut();

          headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
          headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());

          return Ok::<_, Error>(response);
        }
      });

      async { Ok::<_, Error>(svc_fn) }
    });

    let server = Server::bind(&self.address).serve(service);

    info!("🚀 Listening on http://{address}", address = self.address);

    if let Err(err) = server.await {
      error!("Server error: {err:#?}");
    }
  }
}
