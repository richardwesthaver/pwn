use serde::{Serialize, Deserialize};
use crate::Error;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
  GET = 0x00,
  SET = 0x01,

  QUERY = 0x10,
  
  START = 0x20,
  STOP = 0x21,
  SLEEP = 0x22,

  SUSET = 0xF0,
  SUGET = 0xF1,
  SHUTDOWN = 0xFF,
}

impl TryFrom<u8> for OpCode {
  type Error = Error;
  fn try_from(b: u8) -> Result<Self, Error> {
    match b {
      0x00 => Ok(Self::GET),
      0x01 => Ok(Self::SET),
      0x10 => Ok(Self::QUERY),
      0x20 => Ok(Self::START),
      0x21 => Ok(Self::STOP),
      0x22 => Ok(Self::SLEEP),
      0xF0 => Ok(Self::SUSET),
      0xF1 => Ok(Self::SUGET),
      0xFF => Ok(Self::SHUTDOWN),
      e => Err(Error::CodingError(format!("invalid op_code: {}", e))),
    }
  }
}

impl Into<u8> for OpCode {
  fn into(self) -> u8 {
    match self {
      Self::GET => 0x00,
      Self::SET => 0x01,
      Self::QUERY => 0x10,
      Self::START => 0x20,
      Self::STOP => 0x21,
      Self::SLEEP => 0x22,
      Self::SUSET => 0xF0,
      Self::SUGET => 0xF1,
      Self::SHUTDOWN => 0xFF,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[repr(u8)]
pub enum ValType {
  Str,
  Byt,
  Key,
  Enc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Val{
  pub typ: ValType,
  pub len: u32,
  pub val: Vec<u8>,
}
impl From<Vec<u8>> for Val {
  fn from(val: Vec<u8>) -> Self {
    Val{ typ: ValType::Byt, len: val.len() as u32, val }
  }
}

impl From<&[u8]> for Val {
  fn from(v: &[u8]) -> Self {
    Val{ typ: ValType::Byt, len: v.len() as u32, val: v.to_vec() }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  pub typ: OpCode,
  len: u32,
  pub val: Val,
}

impl Message {
  pub fn new(typ: OpCode, len: u32, val: Val) -> Message {
    Message { typ, len, val }
  }
  pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    match bincode::serialize(&self) {
      Ok(res) => Ok(res),
      Err(e) => Err(e.into())
    }
  }
  pub fn typ(&self) -> OpCode {
    self.typ
  }
  pub fn len(&self) -> u32 {
    self.len
  }
  pub fn val(&self) -> &Val {
    &self.val
  }
  pub fn with_typ(mut self, typ: OpCode) -> Self {
    self.typ = typ;
    self
  }
  pub fn with_len(mut self, len: u32) -> Self {
    self.len = len;
    self
  }
  pub fn with_val(mut self, val: Val) -> Self {
    self.val = val;
    self
  }
}
