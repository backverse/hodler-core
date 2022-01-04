mod config;
mod exchange;
mod hodler;

use crate::exchange::binance::binance::Binance;
use crate::exchange::bitkub::bitkub::Bitkub;
use crate::hodler::hodler::Hodler;
use crate::hodler::server::HodlerServer;
use tokio::join;

#[tokio::main]
async fn main() {
  env_logger::init();

  let binance = Binance::new();
  let bitkub = Bitkub::new();
  let hodler = Hodler::new();
  let hodler_server = HodlerServer::new(hodler.clone());

  join!(
    binance.connect_ws(hodler.clone()),
    bitkub.connect_ws(hodler.clone()),
    hodler_server.serve()
  );
}
