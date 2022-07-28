use thiserror::Error;
use std::io;
#[derive(Error, Debug)]
pub enum Error {
    #[error("server disconnected")]
    Disconnect(#[from] io::Error),
    #[error("value error: (expected {expected:?}, found {found:?})")]
    InvalidValue {
        expected: String,
        found: String,
    },
    #[error("unknown client error")]
    Unknown,
}
