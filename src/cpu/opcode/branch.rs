use crate::funct3;

#[derive(Clone, Copy)]
pub enum BranchInstruction {
  Equal,
  NotEqual,
  LessThan,
  GreaterEqual,
  LessThanUnsigned,
  GreaterEqualUnsigned,
}

const CONV_TABLE: [Option<BranchInstruction>; 8] = [
  Some(BranchInstruction::Equal),
  Some(BranchInstruction::NotEqual),
  None,
  None,
  Some(BranchInstruction::LessThan),
  Some(BranchInstruction::GreaterEqual),
  Some(BranchInstruction::LessThanUnsigned),
  Some(BranchInstruction::GreaterEqualUnsigned),
];

impl TryFrom<u32> for BranchInstruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    CONV_TABLE[funct3 as usize].ok_or(())
  }
}
