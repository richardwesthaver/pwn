// TODO this should be replaced with a generic Socket type provided by common lib
use std::net::SocketAddr;
use std::env;
use ed25519_dalek::PublicKey;
use crate::Error;

#[derive(Debug)]
pub struct Cfg {
  pub rx_addr: SocketAddr,
  pub tx_addr: SocketAddr,
  pub database_url: SocketAddr,
  pub client_public_key: PublicKey,
}

impl Cfg {
  pub fn new(rx_addr: SocketAddr, tx_addr: SocketAddr, database_url: SocketAddr, client_public_key: PublicKey) -> Cfg {
    Cfg { rx_addr, tx_addr, database_url, client_public_key }
  }

  // TODO: error types
  pub fn from_env() -> Result<Cfg, std::io::Error> {
    let client_public_key_bytes = proto::base64::decode(&env::var("CLIENT_PUBLIC_KEY").unwrap()).unwrap();
    let client_public_key = PublicKey::from_bytes(&client_public_key_bytes).unwrap();
    Ok(
      Cfg {
	rx_addr: env::var("RX_ADDR").unwrap().parse().unwrap(),
	tx_addr: env::var("TX_ADDR").unwrap().parse().unwrap(),
	database_url: env::var("DATABASE_URL").unwrap().parse().unwrap(),
	client_public_key,
      }
    )
  }

  pub fn from_args(args: Vec<String>) -> Result<Cfg, Error> {
    let client_public_key_bytes = proto::base64::decode(&args[4]).unwrap();
    let client_public_key = PublicKey::from_bytes(&client_public_key_bytes).unwrap();

    Ok(
      Cfg {
	rx_addr: args[1].parse().unwrap(),
	tx_addr: args[2].parse().unwrap(),
	database_url: args[3].parse().unwrap(),
	client_public_key,
      }
    )
  }
}
