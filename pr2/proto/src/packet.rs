use crate::crypto::{PublicKey, Encrypted};
use crate::serialize;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct EncryptedData {
    pub public_key: PublicKey,
    pub data: Encrypted,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Packet {
  pub len: u16,
  pub chk: u32,
  pub val: EncryptedData,
}

impl Packet {
  pub fn len(&self) -> u16 {
    self.len
  }
  pub fn chk(&self) -> u32 {
    self.chk
  }
  pub fn to_bytes(&self) -> Vec<u8> {
    let len_slice = u16::to_le_bytes(self.len());
    let chk_slice = u32::to_le_bytes(self.chk());
    // reserve space in buffer.
    let mut bytes = Vec::with_capacity(6 + self.len() as usize);

    bytes.extend_from_slice(&len_slice);
    bytes.extend_from_slice(&chk_slice);
    bytes.extend_from_slice(&serialize(&self.val).unwrap());
    bytes
  }
}
