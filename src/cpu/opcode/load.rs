use crate::{dev::ReadMode, funct3, map_bits};

#[derive(Clone, Copy)]
pub enum LoadInstruction {
  Byte,
  HalfWord,
  Word,
  DoubleWord,
  ByteUnsigned,
  HalfWordUnsigned,
  WordUnsigned,
}

impl LoadInstruction {
  pub fn conv_loaded_data(&self, data: u64) -> u64 {
    let size = match self {
      Self::Byte => 8,
      Self::HalfWord => 16,
      Self::Word => 32,
      Self::DoubleWord
      | Self::ByteUnsigned
      | Self::HalfWordUnsigned
      | Self::WordUnsigned => return data,
    };

    map_bits! {
      [u64 : data];
      copy [size - 1, 0] => 0;
      repeat size - 1 => [63, size];
    }
  }
}

impl Into<ReadMode> for LoadInstruction {
  fn into(self) -> ReadMode {
    match self {
      Self::Byte | Self::ByteUnsigned => ReadMode::Byte,
      Self::HalfWord | Self::HalfWordUnsigned => ReadMode::HalfWord,
      Self::Word | Self::WordUnsigned => ReadMode::Word,
      Self::DoubleWord => ReadMode::DoubleWord,
    }
  }
}

const CONV_TABLE: [Option<LoadInstruction>; 8] = [
  Some(LoadInstruction::Byte),
  Some(LoadInstruction::HalfWord),
  Some(LoadInstruction::Word),
  Some(LoadInstruction::DoubleWord),
  Some(LoadInstruction::ByteUnsigned),
  Some(LoadInstruction::HalfWordUnsigned),
  Some(LoadInstruction::WordUnsigned),
  None,
];

impl TryFrom<u32> for LoadInstruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    CONV_TABLE[funct3 as usize].ok_or(())
  }
}
