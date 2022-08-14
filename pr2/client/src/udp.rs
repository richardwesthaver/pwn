use proto::{codec::C2Codec, MTU};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct UdpTx {
  socket: UdpSocket,
  codec: C2Codec,
}

impl UdpTx {}
