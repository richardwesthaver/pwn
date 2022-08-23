use std::io;
use thiserror::Error;

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

impl From<rustyline::error::ReadlineError> for Error {
  fn from(err: rustyline::error::ReadlineError) -> Self {
    Error::Internal(err.to_string())
  }
}

impl From<proto::Error> for Error {
  fn from(err: proto::Error) -> Self {
    Error::Internal(err.to_string())
  }
}

impl From<std::string::FromUtf8Error> for Error {
  fn from(err: std::string::FromUtf8Error) -> Self {
    Error::Internal(err.to_string())
  }
}

impl From<tokio::task::JoinError> for Error {
  fn from(err: tokio::task::JoinError) -> Self {
    Error::Internal(err.to_string())
  }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
  fn from(err: tokio::sync::oneshot::error::RecvError) -> Self {
    Error::Internal(err.to_string())
  }
}

impl From<tokio::sync::oneshot::error::TryRecvError> for Error {
  fn from(err: tokio::sync::oneshot::error::TryRecvError) -> Self {
    Error::Internal(err.to_string())
  }
}
