use server::{
  cfg::Cfg,
  db::{insert_agent, list_agents, pg_table_exists, prepare_pg_pool, types::Agent, Pool},
  TxService, RxService
};

use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();
  let cfg = Cfg::from_env().unwrap();

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
    tx_srv.start_http().await.unwrap();
  });

  let rx_task = tokio::spawn(async move {
    let rx_srv = RxService::new(cfg.rx_addr, rx_pool);
    rx_srv.start_rx().await.unwrap();
  });

  tokio::try_join!(rx_task, tx_task,)?;
  log::info!("shutting down..");
  Ok(())
}
