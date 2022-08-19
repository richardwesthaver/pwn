use crate::{
  deserialize,
  packet::{EncryptedData, Packet},
};
use bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

// 8mb
pub const MAX_FRAME_SIZE: usize = 8 * 1024 * 1024;

pub struct C2Codec {}

impl Encoder<Packet> for C2Codec {
  type Error = std::io::Error;
  fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
    // ensure our packet is less than MAX_FRAME_SIZE
    if item.len() as usize > MAX_FRAME_SIZE {
      return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("frame of len {} is too large.", item.len()),
      ));
    }

    let bytes = item.to_bytes();
    dst.extend_from_slice(&bytes);
    Ok(())
  }
}

impl Decoder for C2Codec {
  type Item = Packet;
  type Error = std::io::Error;
  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    if src.len() < 2 {
      // not enough data to read len (2)
      return Ok(None);
    }

    // read the length field
    let mut len_bytes = [0u8; 2];
    len_bytes.copy_from_slice(&src[..2]);
    let len = u16::from_le_bytes(len_bytes);

    if src.len() < 6 {
      // not enough data to read chk (2+4)
      return Ok(None);
    }

    let mut chk_bytes = [0u8; 4];
    chk_bytes.copy_from_slice(&src[2..6]);
    let chk = u32::from_le_bytes(chk_bytes);

    // avoid DoS
    if len as usize > MAX_FRAME_SIZE {
      return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("frame of len {} is too large.", len),
      ));
    }

    if src.len() < 6 + len as usize {
      // full frame hasn't been received yet
      //
      // reserve extra space in rx_buffer
      src.reserve((6 + len - src.len() as u16) as usize);

      return Ok(None);
    }

    // retrieve the val and advance buffer past this frame.
    let val: EncryptedData = deserialize(&src[6..6 + len as usize]).unwrap();
    src.advance(6 + len as usize);

    // decode bytes into packet
    Ok(Some(Packet { len, chk, val }))
  }
}
