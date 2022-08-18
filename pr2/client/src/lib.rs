pub mod cfg;
pub mod error;

pub use error::Error;

use bytes::BytesMut;
use std::{
  io::{stdin, BufRead},
  net::SocketAddr,
};
use tokio::net::UdpSocket;
use tokio_util::{
  codec::{Decoder, Encoder, LinesCodec},
  udp::UdpFramed,
};

pub fn get_stdin_data() -> Result<String, Error> {
  let mut buf = String::new();
  stdin().read_line(&mut buf)?;
  Ok(buf)
}

pub struct Service<'a> {
  input: Box<dyn BufRead + 'a>,
  client: SocketAddr,
  server: SocketAddr,
}

impl<'a> Service<'a> {
  pub fn new<I: BufRead + 'a>(input: I, client: SocketAddr, server: SocketAddr) -> Service<'a> {
    Service {
      input: Box::new(input),
      client,
      server,
    }
  }
  pub async fn start(&self) -> Result<(), Error> {
    let socket = UdpSocket::bind(self.client).await?;
    log::info!("client binding to socket {}", socket.local_addr()?);
    let mut inf = UdpFramed::new(socket, LinesCodec::new());
    let mut buf = BytesMut::new();
    while let Ok(line) = get_stdin_data() {
      inf.codec_mut().encode(line, &mut buf)?;
      inf.get_mut().send_to(&buf, self.server).await?;
    }
    Ok(())
  }
}
