use clap::Parser;
use client::{cfg::Cfg, Error, Service};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Error> {
  // init logging. adjust at runtime with RUST_LOG env var
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::from_default_env()
        // rustyline logging is too verbose, limit to warnings only
        .add_directive("rustyline=warn".parse()?),
    )
    .init();

  // parse configuration from cli and env
  let cfg = Cfg::parse();

  // initialize our service
  let mut srv = Service::new(cfg.client_addr, cfg.server_addr)?;

  // start the service
  srv.start().await?;

  Ok(())
}
