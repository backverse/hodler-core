use super::Hodler;
use hodler::models::price::{BasePrice, Price};
use hyper::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use log::{error, info};
use regex::Regex;
use serde::Serialize;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub struct HodlerServer {
  hodler: Arc<Mutex<Hodler>>,
}

impl HodlerServer {
  pub fn new(hodler: Arc<Mutex<Hodler>>) -> HodlerServer {
    HodlerServer { hodler }
  }

  pub async fn serve(&self) {
    let make_service = make_service_fn(move |_| {
      let hodler = self.hodler.clone();
      let service = service_fn(move |req| {
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

              overview.sort_by(|a, b| b.volume.partial_cmp(&a.volume).unwrap());

              json!(overview)
            }
            (&Method::GET, "/insights") => {
              let raw_query = req.uri().query().unwrap();
              let query_pattern = Regex::new(r"(^s|&s)ymbol=(?P<symbol>\w+)").unwrap();
              let query = query_pattern.captures_iter(raw_query);
              let symbol = match query.last() {
                Some(q) => q["symbol"].to_string(),
                None => {
                  return Ok(
                    Response::builder()
                      .status(StatusCode::NOT_FOUND)
                      .body(Body::empty())
                      .unwrap(),
                  );
                }
              };

              let exchanges = match hodler.prices.clone().get(&symbol) {
                Some(e) => e.clone(),
                None => {
                  return Ok(
                    Response::builder()
                      .status(StatusCode::NOT_FOUND)
                      .body(Body::empty())
                      .unwrap(),
                  );
                }
              };

              #[derive(Serialize)]
              pub struct Insight {
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

              #[derive(Clone, Serialize)]
              pub struct Arbitrage {
                pub exchange: String,
                pub best_routes: String,
                pub rate: f32,
              }

              #[derive(Clone, Serialize)]
              pub struct Premium {
                pub exchange: String,
                pub ask_premium: f32,
                pub ask_price: f32,
                pub bid_premium: f32,
                pub bid_price: f32,
              }

              let mut arbitrages = Vec::<Arbitrage>::new();
              let mut premiums = Vec::<Premium>::new();
              let prices = exchanges.clone().into_values().enumerate();
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
              let mut best_bid_price = exchange.ask_price;
              let mut best_bid_ticker_name = exchange.ticker_name;
              let mut best_arbitrage = 0.0;
              let mut best_ask_premium = 0.0;
              let mut best_bid_premium = 0.0;

              exchanges.for_each(|price| {
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

              prices.for_each(|(i, price)| {
                let arbitrage = Arbitrage {
                  exchange: price.exchange.clone(),
                  best_routes: best_bid_exchange.clone(),
                  rate: (1.0 - price.bid_price / price.ask_price) * 100.0,
                };

                let premium = Premium {
                  exchange: price.exchange,
                  ask_premium: (1.0 - price.ask_price / best_ask_price) * 100.0,
                  ask_price: price.ask_price,
                  bid_premium: (1.0 - price.bid_price / best_bid_price) * 100.0,
                  bid_price: price.bid_price,
                };

                arbitrages.push(arbitrage.clone());
                premiums.push(premium.clone());

                if i == 0 {
                  best_arbitrage = arbitrage.rate;
                  best_ask_premium = premium.ask_premium;
                  best_bid_premium = premium.bid_premium;
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
              });

              let insight = Insight {
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

              json!({
                "insight": insight,
                "arbitrages": arbitrages,
                "premiums": premiums
              })
            }
            (_, _) => {
              return Ok(
                Response::builder()
                  .status(StatusCode::NOT_FOUND)
                  .body(Body::empty())
                  .unwrap(),
              );
            }
          };

          return Ok::<_, Error>(
            Response::builder()
              .header(CONTENT_TYPE, "application/json")
              .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
              .body(Body::from(raw.to_string()))
              .unwrap(),
          );
        }
      });

      return async { Ok::<_, Error>(service) };
    });

    let address = SocketAddr::new("0.0.0.0".parse().unwrap(), config::PORT);
    let server = Server::bind(&address).serve(make_service);

    info!("🚀 Listening on http://{address}");

    if let Err(err) = server.await {
      error!("💀 Server error: {err:#?}");
    }
  }
}
