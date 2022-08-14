use client::{get_stdin_data, Error, Service};
use proto::MTU;
use std::{
  env,
  io::{stdin, Read},
  net::SocketAddr,
};
use tokio::net::UdpSocket;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env().add_directive("trace".parse()?))
    .init();

  // replace with cfg::Cfg
  let remote_addr: SocketAddr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:8080".into())
    .parse()?;

  // ephemerial client port
  let local_addr: SocketAddr = if remote_addr.is_ipv4() {
    "0.0.0.0:0"
  } else {
    "[::]:0"
  }
  .parse()?;

  let srv = Service::new(local_addr, remote_addr);
  srv.start_tx().await?;
  // let tx_task = tokio::spawn(async move {
  //   loop {
  //     let mut data = vec![0u8; MTU];
  //     let (len, other) = socket.recv_from(&mut data).await.unwrap();
  //     if other.eq(&remote_addr) {
  // 	println!("Received {} bytes:\n{}",
  // 		 len,
  // 		 String::from_utf8_lossy(&data[..len])
  // 	);
  //     }
  //   }
  // });
  // tokio::try_join!(tx_task).unwrap();

  Ok(())
}
