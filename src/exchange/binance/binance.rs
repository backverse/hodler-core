use crate::exchange::binance::ticker::Ticker;
use crate::exchange::exchange::Exchange;
use crate::hodler::hodler::Hodler;
use crate::hodler::model::ticker::MarketTicker;
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Clone)]
pub struct Binance {
  name: Exchange,
  ticker_url: String,
}

impl Binance {
  pub fn new() -> Self {
    Self {
      name: Exchange::BinanceEx,
      ticker_url: "wss://stream.binance.com:9443/stream?streams=btcUsdt@ticker/ethUsdt@ticker/dotUsdt@ticker/powrUsdt@ticker/evxUsdt@ticker/manaUsdt@ticker"
        .to_lowercase(),
    }
  }

  pub async fn connect_ws(&self, hodler: Arc<Mutex<Hodler>>) {
    let (stream, _) = connect_async(self.ticker_url.clone()).await.unwrap();
    let (_, read) = stream.split();

    read
      .for_each(|message| async {
        let message = &message.unwrap_or(Message::Close(None));
        let text = if message.is_text() { message } else { return };
        let ticker = Ticker::from_str(text.to_string());

        hodler
          .lock()
          .unwrap()
          .update_market(MarketTicker {
            exchange: self.name.get_name(),
            symbol: ticker.symbol.clone(),
            symbol_key: self.name.get_key(ticker.symbol),
            ask_price: ticker.ask_price,
            bid_price: ticker.bid_price,
          })
          .await;
      })
      .await;
  }
}
