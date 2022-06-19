use super::Hodler;
use hodler::models::currency::Cryptocurrency;
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
            (&Method::GET, "/currencies") => {
              #[derive(Clone, Debug, Serialize)]
              pub struct Currency {
                pub exchange: String,
                pub ask_price: f32,
                pub bid_price: f32,
                pub code: String,
                pub fraction_digits: u8,
                pub updated_at: i64,
              }

              let mut currencies = hodler
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
                  fraction_digits: 8,
                  updated_at: 0,
                },
              );

              json!(currencies)
            }
            (&Method::GET, "/cryptocurrencies") => {
              let cryptocurrencies = hodler
                .cryptocurrencies
                .clone()
                .into_values()
                .map(|exchanges| exchanges.clone().into_values().collect())
                .collect::<Vec<Vec<Cryptocurrency>>>();

              json!(cryptocurrencies)
            }
            (&Method::GET, "/overviews") => {
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

              let mut overviews = hodler
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

              json!(overviews)
            }
            (&Method::GET, "/insights") => {
              let raw_query = req.uri().query().unwrap();
              let query_pattern = Regex::new(r"(^|&)symbol=(?P<symbol>\w+)").unwrap();
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

              let exchanges = match hodler.cryptocurrencies.clone().get(&symbol) {
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
              let mut best_bid_price = exchange.ask_price;
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

              json!({
                "summary": summary,
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
