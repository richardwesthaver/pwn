//! cfg.rs --- configuration
use crate::error::Error;
use clap::Parser;
use std::{net::SocketAddr, str::FromStr};
use url::Url;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TransportType {
  Udp,
  Dns,
  Http,
}

impl TransportType {}

impl FromStr for TransportType {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "udp" | "u" => Ok(Self::Udp),
      "dns" | "d" => Ok(Self::Dns),
      "http" | "h" => Ok(Self::Http),
      _ => Err(Error::InvalidArgument(s.to_string())),
    }
  }
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cfg {
  #[clap(short, long, env, default_value = "127.0.0.1:9053")]
  pub rx_addr: SocketAddr,
  #[clap(short = 's', long, env, default_value = "127.0.0.1:8053")]
  pub tx_addr: SocketAddr,
  #[clap(short, long, env, default_value = "postgres://localhost/pr2.db")]
  pub database_url: Url,
  // this should be a 32-byte base64 encoded string
  #[clap(short, long, env)]
  pub client_public_key: String,
  #[clap(short, long, env, default_value = "udp")]
  pub transport: TransportType,
}
