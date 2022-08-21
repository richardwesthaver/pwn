use crate::Error;
use proto::api::op::{Message, OpCode, Val, ValType};

pub fn parse_line(line: &str) -> Result<Message, Error> {
  let mut words = line.trim_start().split_whitespace();

  let top_str = words
    .next()
    .ok_or(Error::InvalidValue("bad op".to_string()))?;
  let top: OpCode = top_str.parse()?;

  let val_str = words.next().unwrap_or_default();
  let val_top = ValType::Str;
  let val = val_str.to_string().into_bytes();
  let len = val.len() as u32;
  let val: Val = Val {
    top: val_top,
    len,
    val,
  };
  let msg = Message::new(top, len, val);

  log::debug!("{:?}", &msg);

  Ok(msg)
}
