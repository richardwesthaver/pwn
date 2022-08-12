use server::{
  Service,
  cfg::Cfg,
  db::{prepare_pg_pool, Pool, insert_agent, list_agents,
       types::Agent}
};

use tokio::io::Interest;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cfg = Cfg::from_env().unwrap();
  // prepare the postgres database 'pr2.db'
  //  prepare_pg_pool(cfg.db).await?;

  // listens for commands from operator
  let rx = tokio::net::UdpSocket::bind(cfg.rx_addr).await?;

  let pool = Pool::new(cfg.database_url).await?;

//  let agent_id = insert_agent(Agent::default(), &pool).await?;
  let agents: Vec<Agent> = list_agents(&pool).await?;
  for a in agents.iter() {
    println!("{:?}", a);
  }

  //  let tx = tokio::net::TcpListener::bind(cfg.tx_addr).await?;
  // let tx_task = tokio::spawn(async move {
  //   loop {
  //     let (stream, client) = tx.accept().await.unwrap();
  //     println!("accepting client {}", client);
  //     let mut buf = vec![0; 1024];
  //     let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await.unwrap();
  //     if ready.is_readable() {
  // 	match stream.try_read_buf(&mut buf) {
  // 	  Ok(n) => {
  // 	    buf.truncate(n);
  // 	    break;
  // 	  },
  //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
  //           continue;
  //         },
  //         Err(e) => {
  // 	    eprintln!("encountered error {}", e);
  //           return; 
  //         }
  // 	};
  //     }
  //   }
  // });

  let tx_task = tokio::spawn(async move {
    let service = Service::new(pool, cfg);
    service.start_http().await.unwrap();
  });

  let rx_task = tokio::spawn(async move {
    loop {
      let ready = rx.ready(Interest::READABLE | Interest::WRITABLE).await.unwrap();
      if ready.is_readable() {
	let mut buf = vec![0; 1024];
	match rx.try_recv_buf_from(&mut buf) {
	  Ok((n, _client)) => {
	    buf.truncate(n);
	    break;
	  },
          Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            continue;
          },
          Err(e) => {
            return; 
          }
	};
      }
    }
  });

  tokio::try_join!(rx_task,
		   tx_task,
  )?;
  log::info!("shutting down..");
  Ok(())
}
