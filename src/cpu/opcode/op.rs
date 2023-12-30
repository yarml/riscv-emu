#[derive(Clone, Copy)]
pub enum OpInstruction {
  Add,
  Sub,
  ShiftLogicalLeft,
  ShiftLogicalRight,
  ShiftArithmeticRight,
  SetLessThan,
  SetLessThanUnsigned,
  XOR,
  OR,
  AND,
}

const CONV_TABLE_00: [OpInstruction; 8] = [
  OpInstruction::Add,
  OpInstruction::ShiftLogicalLeft,
  OpInstruction::SetLessThan,
  OpInstruction::SetLessThanUnsigned,
  OpInstruction::XOR,
  OpInstruction::ShiftLogicalRight,
  OpInstruction::OR,
  OpInstruction::AND,
];

const CONV_TABLE_01: [Option<OpInstruction>; 8] = [
  Some(OpInstruction::Sub),
  None,
  None,
  None,
  None,
  Some(OpInstruction::ShiftArithmeticRight),
  None,
  None,
];

impl TryFrom<(u32, u32)> for OpInstruction {
  type Error = ();

  fn try_from((funct7, funct3): (u32, u32)) -> Result<Self, Self::Error> {
    match funct7 {
      0b0000000 => Ok(CONV_TABLE_00[funct3 as usize]),
      0b0100000 => CONV_TABLE_01[funct3 as usize].ok_or(()),
      _ => Err(()),
    }
  }
}
