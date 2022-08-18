use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cfg {
  #[clap(short, long, env, default_value = "127.0.0.1:0")]
  pub client_addr: SocketAddr,
  #[clap(short, long, env, default_value = "127.0.0.1:9053")]
  pub server_addr: SocketAddr,
}
