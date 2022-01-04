use crate::exchange::bitkub::ticker::Ticker;
use crate::exchange::exchange::Exchange;
use crate::hodler::hodler::Hodler;
use crate::hodler::model::ticker::MarketTicker;
use futures_util::StreamExt;
use log::error;
use serde_json::from_str;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Clone)]
pub struct Bitkub {
  name: Exchange,
  ticker_url: String,
}

impl Bitkub {
  pub fn new() -> Self {
    Self {
      name: Exchange::BitkubEx.clone(),
      ticker_url: Exchange::BitkubEx.get_ticker_url(),
    }
  }

  pub async fn connect_ws(&self, hodler: Arc<Mutex<Hodler>>) {
    let (stream, _) = connect_async(self.ticker_url.clone()).await.unwrap();
    let (_, read) = stream.split();

    read
      .for_each(|message| async {
        let message = &message.unwrap_or(Message::Close(None));
        let text = if message.is_text() { message } else { return };

        match from_str::<Ticker>(&text.to_string()) {
          Ok(ticker) => {
            hodler
              .lock()
              .unwrap()
              .update_market(MarketTicker {
                exchange: self.name.get_name(),
                symbol: ticker.stream.clone(),
                symbol_key: self.name.get_key(ticker.stream),
                ask_price: ticker.ask_price,
                bid_price: ticker.bid_price,
              })
              .await;
          }
          Err(error) => error!("{:#?}", error),
        };
      })
      .await;
  }
}
