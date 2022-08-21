pub mod c2;
pub mod op;

pub use c2::C2Codec;
pub use op::OpCodec;

pub const MTU: usize = 65_507;
