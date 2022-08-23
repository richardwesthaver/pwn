//! lib.rs --- pr2-client

//! 'Command & Control' is curious terminology, don't you think? Who's
//! really in control anyway?
//!
//! This crate provides the operator UI of pr2. It is a text-based
//! REPL which supports a very basic syntax. Naturally, inspired by
//! Metasploit.
//!
//! Input is parsed and built into a Message containing (op_code, len,
//! val) -- i.e. simple TLV packet. The Message is sent to the server.
//!
//! If the server replies within a given grace period, the Response is
//! decoded and printed.

pub mod cfg;
pub mod error;
pub mod parser;

pub use error::Error;

use proto::codec::OpCodec;

use bytes::BytesMut;
use rustyline::Editor;
use std::{net::SocketAddr, time::Duration};
use tokio::net::UdpSocket;
use tokio_util::{codec::Encoder, udp::UdpFramed};

/// pr2-client runtime service.
pub struct Service {
  input: Editor<()>,
  client: SocketAddr,
  server: SocketAddr,
}

impl Service {
  /// Initialize a service given CLIENT and SERVER sockets.
  pub fn new(client: SocketAddr, server: SocketAddr) -> Result<Service, Error> {
    Ok(Service {
      input: Editor::<()>::new()?,
      client,
      server,
    })
  }

  /// Start the service, running indefinitely or until an
  /// unrecoverable error is encountered.
  pub async fn start(mut self) -> Result<(), Error> {
    // bind to socket
    let socket = UdpSocket::bind(self.client).await?;
    log::info!("client binding to socket {}", socket.local_addr()?);

    // wrap socket with codec
    let mut inf = UdpFramed::new(socket, OpCodec {});

    // set up our tx & rx buffers
    let mut tx_buf = BytesMut::with_capacity(proto::codec::op::MAX_FRAME_SIZE);
    let mut rx_buf = BytesMut::with_capacity(proto::codec::op::MAX_FRAME_SIZE);

    // loop over input
    while let Ok(line) = &self.input.readline("|| ") {
      match parser::parse_line(&line) {
        // message parsed
        Ok(Some(m)) => {
          // encode message in tx_buf
          inf.codec_mut().encode(m, &mut tx_buf)?;

          // wait for socket to be writable
          inf.get_ref().writable().await?;
          // send
          inf.get_mut().send_to(&tx_buf, &self.server).await?;

          // receive a response
          // TODO: configurable timeout
          std::thread::sleep(Duration::from_millis(200));
          loop {
            if let Ok((n, client)) = inf.get_mut().try_recv_buf_from(&mut rx_buf) {
              // ensure we're talking to the same server, then process response
              if client == self.server {
                log::trace!("RX {} FROM {}", n, client);

                // parse as string
                let res = String::from_utf8(rx_buf.to_vec())?;

                // process newline chars
                // TODO: support other control chars? \n\r?
                for i in res.chars() {
                  if i == '\n' {
                    println!("")
                  } else {
                    print!("{}", i)
                  }
                }
                break;
              } else {
                // warn user of mitm encounter
                log::warn!("UNSOLICITED RX {} FROM {}", n, client);
              }
            }
          }
        }
        // no input
        Ok(None) => continue,
        // invalid message
        Err(e) => {
          eprintln!("error: {e}");
        }
      }

      // clear buffer in preparation for next iteration
      tx_buf.clear();
      rx_buf.clear();
    }
    Ok(())
  }
}
