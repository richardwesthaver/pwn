use crate::{serialize, Error};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
  GET = 0x00,
  SET = 0x01,
  QUERY = 0x10,
  START = 0x20,
  STOP = 0x21,
  SLEEP = 0x22,
  SUGET = 0xF0,
  SUSET = 0xF1,
  SHUTDOWN = 0xFF,
}

impl std::fmt::Display for OpCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::GET => f.write_str("GET"),
      Self::SET => f.write_str("SET"),
      Self::QUERY => f.write_str("QUERY"),
      Self::START => f.write_str("START"),
      Self::STOP => f.write_str("STOP"),
      Self::SLEEP => f.write_str("SLEEP"),
      Self::SUGET => f.write_str("SUSET"),
      Self::SUSET => f.write_str("SUGET"),
      Self::SHUTDOWN => f.write_str("SHUTDOWN"),
    }
  }
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
      0xF0 => Ok(Self::SUGET),
      0xF1 => Ok(Self::SUSET),
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
      Self::SUGET => 0xF0,
      Self::SUSET => 0xF1,
      Self::SHUTDOWN => 0xFF,
    }
  }
}

impl FromStr for OpCode {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_uppercase().as_str() {
      "GET" => Ok(Self::GET),
      "SET" => Ok(Self::SET),
      "QUERY" => Ok(Self::QUERY),
      "START" => Ok(Self::START),
      "STOP" => Ok(Self::STOP),
      "SLEEP" => Ok(Self::SLEEP),
      "SUSET" => Ok(Self::SUGET),
      "SUGET" => Ok(Self::SUSET),
      "SHUTDOWN" => Ok(Self::SHUTDOWN),
      e => Err(Error::CodingError(e.to_string())),
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

impl std::fmt::Display for ValType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Str => f.write_str("str"),
      Self::Byt => f.write_str("byt"),
      Self::Key => f.write_str("key"),
      Self::Enc => f.write_str("enc"),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Val {
  pub top: ValType,
  pub len: u32,
  pub val: Vec<u8>,
}

impl From<Vec<u8>> for Val {
  fn from(val: Vec<u8>) -> Self {
    Val {
      top: ValType::Byt,
      len: val.len() as u32,
      val,
    }
  }
}

impl From<&[u8]> for Val {
  fn from(v: &[u8]) -> Self {
    Val {
      top: ValType::Byt,
      len: v.len() as u32,
      val: v.to_vec(),
    }
  }
}

impl std::fmt::Display for Val {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "[top: {}, len: {}, val: {:?}]",
      self.top, self.len, self.val
    ))
  }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  /// type/OpCode
  top: OpCode,
  /// total Message length
  len: u32,
  /// Value container
  val: Val,
}

impl Message {
  pub fn new(top: OpCode, len: u32, val: Val) -> Message {
    Message { top, len, val }
  }
  pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    let top = u8::to_le_bytes(self.top() as u8);
    let len = u32::to_le_bytes(self.len());
    let mut bytes = Vec::with_capacity(5 + self.len() as usize);
    bytes.extend_from_slice(&top);
    bytes.extend_from_slice(&len);
    bytes.extend_from_slice(&serialize(&self.val)?);
    Ok(bytes)
  }
  pub fn top(&self) -> OpCode {
    self.top
  }
  pub fn len(&self) -> u32 {
    self.len
  }
  pub fn val(&self) -> &Val {
    &self.val
  }
  pub fn with_top(mut self, top: OpCode) -> Self {
    self.top = top;
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

impl std::fmt::Display for Message {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "TOP: {}, LEN: {}, VAL: {}",
      self.top, self.len, self.val
    ))
  }
}
