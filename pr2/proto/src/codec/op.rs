use crate::{
  Error,
  api::op::{Message, OpCode},
};
use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

/// 120mb
pub const MAX_FRAME_SIZE: usize = 120 * 1024 * 1024;

/// OP <--> C2 codec driver
#[derive(Copy, Clone, Debug)]
pub struct OpCodec {}

impl Encoder<Message> for OpCodec {
  type Error = Error;
  fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
    // protect from DoS
    if item.len() as usize > MAX_FRAME_SIZE {
      return Err(Error::CodingError(format!(
        "frame of len {} is too large.",
        item.len()
      )));
    }
    let bytes = item.to_bytes()?;
    dst.extend_from_slice(&bytes);
    Ok(())
  }
}

impl Decoder for OpCodec {
  type Item = Message;
  type Error = Error;
  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    if src.len() < 1 {
      // not enough data to read top (u8)
      return Ok(None);
    }

    // read the typ byte (opcode)
    let op_code = OpCode::try_from(src[0])?;

    if src.len() < 5 {
      // not enough data to read len (u32)
      return Ok(None);
    }
    let mut len_bs = [0u8; 4];
    len_bs.copy_from_slice(&src[1..5]);
    let len = u32::from_le_bytes(len_bs);
    if len as usize > MAX_FRAME_SIZE {
      return Err(Error::CodingError(format!(
        "frame of len {} is too large.",
        len
      )));
    }

    if src.len() < len as usize {
      // full frame hasn't been received yet
      //
      // reserve extra space in rx_buffer
      //
      // TODO: better buffer management strategy
      src.reserve((len - src.len() as u32) as usize);
      return Ok(None);
    }

    // src.advance(len as usize);
    //    let val = &src[..len as usize];
    Ok(Some(Message::new(op_code, len, &src[5..])))
  }
}
