use proto::{MTU, codec::C2Codec};
use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub struct UdpTx {
  socket: UdpSocket,
  codec: C2Codec,
}

impl UdpTx {

}
