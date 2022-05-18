pub mod ticker;

use crate::exchange::Exchange;
use futures_util::stream::{iter, SplitStream};
use futures_util::{SinkExt, StreamExt};
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
      name: Exchange::FtxEx.clone(),
      ticker_url: Exchange::FtxEx.get_ticker_url(),
    }
  }

  pub async fn connect_ws(&self) -> SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let (stream, _) = connect_async(self.ticker_url.clone()).await.unwrap();
    let (mut write, read) = stream.split();
    let mut tickers = iter(
      Exchange::FtxEx
        .get_tickers()
        .into_iter()
        .map(|t| Ok(t.into())),
    );

    write.send_all(&mut tickers).await.unwrap();

    return read;
  }
}
