use crate::Error;
use proto::api::op::{Message, OpCode};

static HELP: &'static str = r#"
# data
get k
set k v
query 'select a from b order by c'
# proc
start id
stop id
sleep s
# unsafe
suget K
suset K V
shutdown"#;

/// parse a LINE
pub fn parse_line(line: &str) -> Result<Option<Message>, Error> {
  if line.is_empty() {
    return Ok(None);
  }

  let line = line.trim();

  // a value was provided, split to (op, val)
  if let Some((op, val)) = line.split_once(' ') {
    let top: OpCode = op
      .parse()
      .map_err(|_| Error::InvalidValue(HELP.to_string()))?;
    let val = val.as_bytes();
    let len = val.len() as u32 + 5;
    let msg = Message::new(top, len, val);
    log::debug!("{:?}", &msg);
    Ok(Some(msg))
  } else if let Some(op) = line.split_whitespace().next() {
    let top: OpCode = op
      .parse()
      .map_err(|_| Error::InvalidValue(HELP.to_string()))?;
    let val = &[];
    let len = 5;
    let msg = Message::new(top, len, val);
    return Ok(Some(msg));
  } else {
    return Ok(None);
  }
}
