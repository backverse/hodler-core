use super::Hodler;
use crate::hodler::models::oracle::OracleJson;
use crate::hodler::models::price::BasePrice;
use hyper::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
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
