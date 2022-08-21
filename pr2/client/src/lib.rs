//! lib.rs --- pr2-client

pub mod cfg;
pub mod error;
pub mod parser;

pub use error::Error;

use proto::codec::OpCodec;

use bytes::BytesMut;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio_util::{codec::Encoder, udp::UdpFramed};

use rustyline::Editor;

pub struct Service {
  input: Editor<()>,
  client: SocketAddr,
  server: SocketAddr,
}

impl Service {
  pub fn new(client: SocketAddr, server: SocketAddr) -> Result<Service, Error> {
    Ok(Service {
      input: Editor::<()>::new()?,
      client,
      server,
    })
  }
  pub async fn start(&mut self) -> Result<(), Error> {
    let socket = UdpSocket::bind(self.client).await?;
    log::info!("client binding to socket {}", socket.local_addr()?);
    let mut inf = UdpFramed::new(socket, OpCodec {});
    let mut buf = BytesMut::new();
    while let Ok(line) = self.input.readline("|| ") {
      match parser::parse_line(&line) {
        Ok(m) => {
          inf.codec_mut().encode(m, &mut buf)?;
          inf.get_mut().send_to(&buf, self.server).await?;
        }
        Err(e) => {
          eprintln!("error: {e}");
        }
      }
      buf.clear();
    }
    Ok(())
  }
}
