use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum Error {
  #[error("server disconnected")]
  Disconnect(#[from] io::Error),
  #[error("`val: {0}")]
  InvalidValue(String),
  #[error("{0}")]
  Internal(String),
}

impl From<std::net::AddrParseError> for Error {
  fn from(err: std::net::AddrParseError) -> Self {
    Error::InvalidValue(err.to_string())
  }
}

impl From<tracing_subscriber::filter::ParseError> for Error {
  fn from(err: tracing_subscriber::filter::ParseError) -> Self {
    Error::InvalidValue(err.to_string())
  }
}

impl From<tokio_util::codec::LinesCodecError> for Error {
  fn from(err: tokio_util::codec::LinesCodecError) -> Self {
    Error::InvalidValue(err.to_string())
  }
}
