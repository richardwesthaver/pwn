pub mod error;
pub mod cfg;
pub mod udp;
pub use error::Error;
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use proto::codec::C2Codec;
use tokio_util::codec::{Decoder, Encoder};
use tokio_util::codec::LinesCodec;
use tokio_util::udp::UdpFramed;
use bytes::BytesMut;
use std::io::{stdin, Read};
use futures::{Stream, Sink};

use proto::packet::Packet;

pub fn get_stdin_data() -> Result<String, Error> {
  let mut buf = String::new();
  stdin().read_line(&mut buf)?;
  Ok(buf)
}

pub struct Service {
  socket: SocketAddr,
  remote: SocketAddr,
}

impl Service {
  pub fn new(socket: SocketAddr, remote: SocketAddr) -> Service {
    Service{socket, remote}
  }
  pub async fn start_tx(&self) -> Result<(), Error> {
    let socket = UdpSocket::bind(self.socket).await?;
    log::info!("client binding to socket {}", socket.local_addr()?);
    let mut inf = UdpFramed::new(socket, LinesCodec::new());
    let mut buf = BytesMut::new();
    while let Ok(line) = get_stdin_data() {
      inf.codec_mut().encode(line, &mut buf)?;
      inf.get_mut().send_to(&buf, self.remote).await?;
    }
    Ok(())
  }
}
