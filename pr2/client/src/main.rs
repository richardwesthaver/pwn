use clap::Parser;
use client::{cfg::Cfg, get_stdin_data, Error, Service};
use std::io;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Error> {
  // init logging. adjust at runtime with RUST_LOG env var
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env().add_directive("trace".parse()?))
    .init();

  // parse configuration from cli and env
  let cfg = Cfg::parse();

  // wrap our input (stdin) in a BufReader to satisfy impl BufRead
  // constraint
  let input = io::BufReader::new(io::stdin());

  // initialize our service
  let srv = Service::new(input, cfg.client_addr, cfg.server_addr);

  // start the service
  srv.start().await?;

  Ok(())
}
