use crate::funct3;

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