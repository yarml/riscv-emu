use crate::funct3;

#[derive(Clone, Copy)]
pub enum StoreInstruction {
  Byte,
  HalfWord,
  Word,
  DoubleWord,
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
