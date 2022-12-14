//! lib.rs --- proto
//!
//! Common types and traits used by `server', `agent', and `client'.

pub mod api;
pub mod codec;
pub mod crypto;
pub mod error;
pub use error::{Error, Result};

pub use base32;
pub use base58;
pub use base64;
pub use bincode::{deserialize, serialize};
pub use hex;
