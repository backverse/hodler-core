pub mod ticker;

use crate::exchange::Exchange;
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[derive(Clone)]
pub struct Client {
  pub name: Exchange,
  ticker_url: String,
}

impl Client {
  pub fn new() -> Self {
    Self {
      name: Exchange::BinanceEx,
      ticker_url: Exchange::BinanceEx.get_ticker_url(),
    }
  }

  pub async fn connect_ws(&self) -> SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let (stream, _) = connect_async(self.ticker_url.clone()).await.unwrap();
    let (_, read) = stream.split();

    return read;
  }
}
