#[derive(Clone, Copy)]
pub enum OpImmInstruction {
  Add,
  ShiftLogicalLeft,
  ShiftLogicalRight,
  ShiftArithmeticRight,
  SetLessThan,
  SetLessThanUnsigned,
  XOR,
  OR,
  AND,
}

// If value is None, means check the CONV_TABLE2_* depending on imm_6_11
const CONV_TABLE: [Option<OpImmInstruction>; 8] = [
  Some(OpImmInstruction::Add),
  None,
  Some(OpImmInstruction::SetLessThan),
  Some(OpImmInstruction::SetLessThanUnsigned),
  Some(OpImmInstruction::XOR),
  None,
  Some(OpImmInstruction::OR),
  Some(OpImmInstruction::AND),
];

const CONV_TABLE2_00: [Option<OpImmInstruction>; 8] = [
  None,
  Some(OpImmInstruction::ShiftLogicalLeft),
  None,
  None,
  None,
  Some(OpImmInstruction::ShiftLogicalRight),
  None,
  None,
];

const CONV_TABLE2_01: [Option<OpImmInstruction>; 8] = [
  None,
  None,
  None,
  None,
  None,
  Some(OpImmInstruction::ShiftArithmeticRight),
  None,
  None,
];

impl TryFrom<(u32, u32)> for OpImmInstruction {
  type Error = ();

  fn try_from((imm_6_11, funct3): (u32, u32)) -> Result<Self, Self::Error> {
    match CONV_TABLE[funct3 as usize] {
      Some(inst) => Ok(inst),
      None => {
        if imm_6_11 == 0 {
          CONV_TABLE2_00[funct3 as usize].ok_or(())
        } else if imm_6_11 == 0b010000 {
          CONV_TABLE2_01[funct3 as usize].ok_or(())
        } else {
          Err(())
        }
      }
    }
  }
}
