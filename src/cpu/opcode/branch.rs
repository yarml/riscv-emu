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

impl BranchInstruction {
  pub fn check(&self, rs1v_u: u64, rs2v_u: u64) -> bool {
    let rs1v = rs1v_u as i64;
    let rs2v = rs2v_u as i64;
    match self {
      BranchInstruction::Equal => rs1v_u == rs2v_u,
      BranchInstruction::NotEqual => rs1v_u != rs2v_u,
      BranchInstruction::LessThan => rs1v < rs2v,
      BranchInstruction::GreaterEqual => rs1v >= rs2v,
      BranchInstruction::LessThanUnsigned => rs1v_u < rs2v_u,
      BranchInstruction::GreaterEqualUnsigned => rs1v_u >= rs2v_u,
    }
  }
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
