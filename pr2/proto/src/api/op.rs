use serde::{Serialize, Deserialize};
use crate::Error;
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ValType {
  Str,
  Byt,
  Key,
  Enc,
  
}

#[derive(Debug, Serialize, Deserialize)]
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
  pub len: u32,
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
  pub fn typ(mut self, typ: OpCode) -> Self {
    self.typ = typ;
    self
  }
  pub fn len(mut self, len: u32) -> Self {
    self.len = len;
    self
  }
  pub fn val(mut self, val: Val) -> Self {
    self.val = val;
    self
  }
}
