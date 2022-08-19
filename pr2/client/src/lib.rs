pub mod cfg;
pub mod error;

pub use error::Error;

use bytes::BytesMut;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio_util::{
  codec::{Decoder, Encoder, LinesCodec},
  udp::UdpFramed,
};

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
    let mut inf = UdpFramed::new(socket, LinesCodec::new());
    let mut buf = BytesMut::new();
    while let Ok(line) = self.input.readline("|| ") {
      inf.codec_mut().encode(line, &mut buf)?;
      inf.get_mut().send_to(&buf, self.server).await?;
      buf.clear();
    }
    Ok(())
  }
}
