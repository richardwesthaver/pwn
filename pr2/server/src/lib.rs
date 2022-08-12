pub mod cfg;
pub mod udp;
pub mod db;
pub mod error;
#[cfg(feature = "http")]
pub mod http;

pub use cfg::Cfg;
pub use db::Pool;
pub use error::Error;

use proto::codec::C2Codec;
use std::sync::Arc;
use tokio::net::UdpSocket;
use bytes::{BufMut, BytesMut};
use tokio_util::udp::UdpFramed;

#[derive(Debug)]
pub struct Service {
  cfg: Cfg,
  pool: Pool
}

impl Service {
  pub fn new(pool: Pool, cfg: Cfg) -> Service {
    Service { cfg, pool }
  }

  pub async fn start_rx(&self) -> Result<(), Error> {
    let socket = UdpSocket::bind(self.cfg.rx_addr).await?;
    let inf = UdpFramed::new(socket, C2Codec {});
    let mut buf = &mut BytesMut::new();
    Ok(())
  }

  #[cfg(feature = "http")]
  pub async fn start_http(self) -> Result<(), Error> {
    let tx_addr = self.cfg.tx_addr;
    log::info!("starting server on: {}", tx_addr);
    let app_state = Arc::new(http::AppState::new(self));
    let routes = http::routes(app_state);
    warp::serve(routes).bind(tx_addr).await;
    Ok(())
  }

  #[cfg(feature = "dns")]
  pub async fn start_dns(&self) -> Result<(), Error> {
    Ok(())
  }
}
