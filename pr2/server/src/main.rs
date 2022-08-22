use clap::Parser;
use server::{
  cfg::{Cfg, TransportType},
  db::{list_agents, prepare_pg_pool, types::Agent, Pool},
  RxService, TxService,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();
  let cfg = Cfg::parse();

  // connect to psql
  let tx_pool = Pool::new(&cfg.database_url).await?;
  let rx_pool = Pool::new(&cfg.database_url).await?;
  // initialize the database if needed
  prepare_pg_pool(&tx_pool).await?;

  //  let agent_id = insert_agent(Agent::default(), &pool).await?;
  let agents: Vec<Agent> = list_agents(&rx_pool).await?;
  for a in agents.iter() {
    println!("{:?}", a);
  }

  let tx_task = tokio::spawn(async move {
    let tx_srv = TxService::new(cfg.tx_addr, tx_pool);
    if cfg.transport == TransportType::Udp {
      tx_srv.start_udp().await.unwrap();
    } else if cfg.transport == TransportType::Dns {
      tx_srv.start_dns().await.unwrap();
    } else if cfg.transport == TransportType::Http {
      tx_srv.start_http().await.unwrap();
    }
  });

  let rx_task = tokio::spawn(async move {
    let rx_srv = RxService::new(cfg.rx_addr, rx_pool);
    rx_srv.start_rx().await.expect("rx_thread terminated");
  });

  tokio::try_join!(rx_task, tx_task)?;
  log::info!("shutting down..");
  Ok(())
}
