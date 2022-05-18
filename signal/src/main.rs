mod hodler;
mod server;

use crate::hodler::Hodler;
use crate::{hodler::models::ticker::MarketTicker, server::HodlerServer};
use exchange::{binance, bitkub, ftx};
use futures_util::StreamExt;
use log::error;
use serde_json::from_str;
use tokio::join;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
  env_logger::init();
  let hodler = Hodler::new();
  let hodler_server = HodlerServer::new(hodler.clone());

  let binance = binance::Client::new();
  let binance_ws_handler = binance.connect_ws().await.for_each(|message| async {
    let message = &message.unwrap_or(Message::Close(None));
    let text = if message.is_text() { message } else { return };

    match from_str::<binance::ticker::Ticker>(&text.to_string()) {
      Ok(ticker) => {
        hodler
          .lock()
          .unwrap()
          .update_market(MarketTicker {
            exchange: binance.name.get_name(),
            symbol: ticker.symbol.clone(),
            symbol_key: binance.name.get_key(ticker.symbol),
            ask_price: ticker.ask_price,
            bid_price: ticker.bid_price,
          })
          .await;
      }
      Err(error) => error!(target: &binance.name.get_name(), "{error:?}: {message:?}"),
    };
  });

  let bitkub = bitkub::Client::new();
  let bitkub_ws_handler = bitkub.connect_ws().await.for_each(|message| async {
    let message = &message.unwrap_or(Message::Close(None));
    let text = if message.is_text() { message } else { return };

    match from_str::<bitkub::ticker::Ticker>(&text.to_string()) {
      Ok(ticker) => {
        hodler
          .lock()
          .unwrap()
          .update_market(MarketTicker {
            exchange: bitkub.name.get_name(),
            symbol: ticker.symbol.clone(),
            symbol_key: bitkub.name.get_key(ticker.symbol),
            ask_price: ticker.ask_price,
            bid_price: ticker.bid_price,
          })
          .await;
      }
      Err(error) => error!(target: &bitkub.name.get_name(), "{error:?}: {message:?}"),
    }
  });

  let ftx = ftx::Client::new();
  let ftx_ws_handler = ftx.connect_ws().await.for_each(|message| async {
    let message = &message.unwrap_or(Message::Close(None));
    let is_ticker = message.is_text() && message.to_string().contains("type\":\"update");
    let text = if is_ticker { message } else { return };

    match from_str::<ftx::ticker::Ticker>(&text.to_string()) {
      Ok(ticker) => {
        hodler
          .lock()
          .unwrap()
          .update_market(MarketTicker {
            exchange: ftx.name.get_name(),
            symbol: ticker.symbol.clone(),
            symbol_key: ftx.name.get_key(ticker.symbol),
            ask_price: ticker.ask_price,
            bid_price: ticker.bid_price,
          })
          .await;
      }
      Err(error) => error!(target: &ftx.name.get_name(), "{error:?}: {message:?}"),
    }
  });

  join!(
    binance_ws_handler,
    bitkub_ws_handler,
    ftx_ws_handler,
    hodler_server.serve()
  );
}
