use bincode::Error as BincodeError;
use chacha20poly1305::aead::Error as AeadError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
  #[error("Encryption error")]
  EncryptionError(String),
  #[error("{0}")]
  SerializationError(String),
  #[error("{0}")]
  CodingError(String),
}

impl From<AeadError> for Error {
  fn from(err: AeadError) -> Error {
    Error::EncryptionError(err.to_string())
  }
}

impl From<BincodeError> for Error {
  fn from(err: BincodeError) -> Error {
    Error::SerializationError(err.to_string())
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::CodingError(err.to_string())
  }
}
