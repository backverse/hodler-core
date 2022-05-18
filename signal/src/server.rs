use super::Hodler;
use crate::hodler::models::oracle::OracleJson;
use crate::hodler::models::price::BasePrice;
use hyper::header::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use log::{error, info};
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
      address: "0.0.0.0:3000".parse().unwrap(),
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
            (&Method::GET, "/bases") => {
              let base_prices = hodler
                .bases
                .clone()
                .into_values()
                .collect::<Vec<BasePrice>>();

              json!(base_prices)
            }
            (&Method::GET, "/oracles") => {
              let oracles = hodler
                .oracles
                .clone()
                .into_values()
                .map(|oracle| oracle.to_json())
                .collect::<Vec<OracleJson>>();

              json!(oracles)
            }
            (_, _) => {
              let mut response = Response::new(Body::empty());

              *response.status_mut() = StatusCode::NOT_FOUND;

              return Ok::<_, Error>(response);
            }
          };

          let body = Body::from(raw.to_string());
          let mut response = Response::builder().body(body).unwrap();

          response.headers_mut().insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
          );

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
