mod exchange;
mod hodler;

use exchange::binance::binance::Binance;
use exchange::bitkub::bitkub::Bitkub;
use hodler::hodler::Hodler;
use mini_redis::client;
use tokio::join;

#[tokio::main]
async fn main() {
  env_logger::init();

  let binance = Binance::new();
  let bitkub = Bitkub::new();
  let redis = client::connect("127.0.0.1:6379").await.unwrap();
  let hodler = Hodler::new(redis);

  join!(
    binance.connect_ws(hodler.clone()),
    bitkub.connect_ws(hodler.clone()),
  );
}
