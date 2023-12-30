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

  fn try_from(funct: u32) -> Result<Self, Self::Error> {
    CONV_TABLE[funct as usize].ok_or(())
  }
}
