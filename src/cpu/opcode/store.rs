use std::num::Wrapping;

use crate::{funct3, dev::WriteMode, sel_word, sel_hword, sel_byte};

#[derive(Clone, Copy)]
pub enum StoreInstruction {
  Byte,
  HalfWord,
  Word,
  DoubleWord,
}

impl StoreInstruction {
  pub fn into_write_mode(&self, data: u64) -> WriteMode {
    match self {
        Self::Byte => WriteMode::Byte(Wrapping(sel_byte!(data, 0) as u8)),
        Self::HalfWord => WriteMode::HalfWord(Wrapping(sel_hword!(data, 0) as u16)),
        Self::Word => WriteMode::Word(Wrapping(sel_word!(data, 0) as u32)),
        Self::DoubleWord => WriteMode::DoubleWord(Wrapping(data)),
    }
  }
}

const CONV_TABLE: [Option<StoreInstruction>; 8] = [
  Some(StoreInstruction::Byte),
  Some(StoreInstruction::HalfWord),
  Some(StoreInstruction::Word),
  Some(StoreInstruction::DoubleWord),
  None,
  None,
  None,
  None,
];

impl TryFrom<u32> for StoreInstruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    CONV_TABLE[funct3 as usize].ok_or(())
  }
}
