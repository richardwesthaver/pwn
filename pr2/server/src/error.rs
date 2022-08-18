//! error.rs --- error types
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
  #[error("Internal error")]
  Internal(String),
  #[error("{0}")]
  NotFound(String),
  #[error("{0}")]
  InvalidArgument(String),
  #[error("authentication required")]
  AuthenticationRequired,
  #[error("{0}")]
  PermissionDenied(String),
  #[error("{0}")]
  Conflict(String),
}

#[cfg(feature = "http")]
impl warp::reject::Reject for Error {}

impl std::convert::From<sqlx::migrate::MigrateError> for Error {
  fn from(err: sqlx::migrate::MigrateError) -> Self {
    Error::Internal(err.to_string())
  }
}

impl std::convert::From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::Internal(err.to_string())
  }
}

impl std::convert::From<std::num::ParseIntError> for Error {
  fn from(err: std::num::ParseIntError) -> Self {
    Error::InvalidArgument(err.to_string())
  }
}

impl std::convert::From<std::env::VarError> for Error {
  fn from(err: std::env::VarError) -> Self {
    Error::InvalidArgument(err.to_string())
  }
}

impl std::convert::From<std::net::AddrParseError> for Error {
  fn from(err: std::net::AddrParseError) -> Self {
    Error::InvalidArgument(err.to_string())
  }
}

impl std::convert::From<url::ParseError> for Error {
  fn from(err: url::ParseError) -> Self {
    Error::InvalidArgument(err.to_string())
  }
}
impl std::convert::From<sqlx::Error> for Error {
  fn from(err: sqlx::Error) -> Self {
    match err {
      sqlx::Error::RowNotFound => Error::NotFound("Not found".into()),
      _ => Error::Internal(err.to_string()),
    }
  }
}

impl std::convert::From<ed25519_dalek::SignatureError> for Error {
  fn from(err: ed25519_dalek::SignatureError) -> Self {
    Error::Internal(err.to_string())
  }
}

impl std::convert::From<proto::base64::DecodeError> for Error {
  fn from(err: proto::base64::DecodeError) -> Self {
    Error::Internal(err.to_string())
  }
}

impl std::convert::From<proto::Error> for Error {
  fn from(err: proto::Error) -> Self {
    match err {
      proto::Error::EncryptionError(s) => Error::Internal(s),
      proto::Error::SerializationError(s) => Error::Internal(s),
    }
  }
}
