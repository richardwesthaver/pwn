use proto::{MTU, codec::C2Codec};
use tokio::net::UdpSocket;
use tokio_util::udp::UdpFramed;

pub struct UdpRx {
  socket: UdpSocket,
  codec: C2Codec,
}

impl UdpRx {

}

pub struct UdpTx {
  socket: UdpSocket,
}
