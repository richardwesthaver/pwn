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
  let mut words = line.trim_start().split_whitespace();
  let top_str = words
    .next()
    .ok_or(Error::InvalidValue("bad op".to_string()))?;
  let top: OpCode = top_str
    .parse()
    .map_err(|_| Error::InvalidValue(HELP.to_string()))?;
  let maybe_val = words.next();
  let val = if let Some(v) = maybe_val {
    v.as_bytes()
  } else {
    &[]
  };

  let len = val.len() as u32 + 5;

  let msg = Message::new(top, len, val);

  log::debug!("{:?}", &msg);

  Ok(Some(msg))
}
