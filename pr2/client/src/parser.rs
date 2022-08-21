use crate::Error;
use proto::api::op::{Message, OpCode, Val, ValType};

static HELP: &'static str = r#"
+----------+-------------+----+ 
| OP       | ARGS        | ?? | 
|----------+-------------+----| 
| GET      | [KEY]       |    | 
| SET      | [KEY] [VAL] |    | 
| QUERY    | [SQL]       |    | 
| START    | [ID]        |    | 
| STOP     | [ID]        |    | 
| SLEEP    | [SEC]       |    | 
|----------+-------------+----| 
| UNSAFE                      | 
|----------+-------------+----| 
| SUGET    | [KEY]       |    | 
| SUSET    | [KEY] [VAL] |    | 
| SHUTDOWN | nil         |    | 
\----------+-------------+----/ 
"#;

pub fn parse_line(line: &str) -> Result<Message, Error> {
  let mut words = line.trim_start().split_whitespace();
  let top_str = words
    .next()
    .ok_or(Error::InvalidValue("bad op".to_string()))?;
  let top: OpCode = top_str
    .parse()
    .map_err(|_| Error::InvalidValue(HELP.to_string()))?;
  let val_str = words
    .next()
    .ok_or(Error::InvalidValue("bad val".to_string()))?;
  let val_top = ValType::Str;
  let val = val_str.as_bytes().to_vec();
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
