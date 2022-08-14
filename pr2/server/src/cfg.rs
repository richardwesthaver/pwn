//! cfg.rs --- configuration
use crate::Error;
use ed25519_dalek::PublicKey;
use std::{env, net::SocketAddr};
use std::net::ToSocketAddrs;
use url::Url;

#[derive(Debug, Clone)]
pub struct Cfg {
  pub rx_addr: SocketAddr,
  pub tx_addr: SocketAddr,
  pub database_url: Url,
  pub client_public_key: PublicKey,
}

impl Cfg {
  pub fn new<U: Into<Url>>(
    rx_addr: SocketAddr,
    tx_addr: SocketAddr,
    database_url: U,
    client_public_key: PublicKey,
  ) -> Cfg {
    let database_url = database_url.into();
    Cfg {
      rx_addr,
      tx_addr,
      database_url,
      client_public_key,
    }
  }

  pub fn from_env() -> Result<Cfg, Error> {
    let client_public_key_bytes = proto::base64::decode(&env::var("CLIENT_PUBLIC_KEY")?)?;
    let client_public_key = PublicKey::from_bytes(&client_public_key_bytes)?;
    Ok(Cfg {
      rx_addr: env::var("RX_ADDR")?.to_socket_addrs()?.next().unwrap_or(SocketAddr::V4(std::net::SocketAddrV4::new([127,0,0,1].into(), 9053))),
      tx_addr: env::var("TX_ADDR")?.to_socket_addrs()?.next().unwrap_or(SocketAddr::V4(std::net::SocketAddrV4::new([127,0,0,1].into(), 8053))),
      database_url: env::var("DATABASE_URL")?.parse()?,
      client_public_key,
    })
  }

  pub fn from_args(args: Vec<String>) -> Result<Cfg, Error> {
    let client_public_key_bytes = proto::base64::decode(&args[4])?;
    let client_public_key = PublicKey::from_bytes(&client_public_key_bytes)?;
    Ok(Cfg {
      rx_addr: args[1].parse()?,
      tx_addr: args[2].parse()?,
      database_url: args[3].parse()?,
      client_public_key,
    })
  }
}
