use crate::{funct3, funct7};

#[derive(Clone, Copy)]
pub enum Op32Instruction {
  Add,
  Sub,
  ShiftLogicalLeft,
  ShiftLogicalRight,
  ShiftArithmeticRight,
}

const CONV_TABLE_00: [Option<Op32Instruction>; 8] = [
  Some(Op32Instruction::Add),
  Some(Op32Instruction::ShiftLogicalLeft),
  None,
  None,
  None,
  Some(Op32Instruction::ShiftLogicalRight),
  None,
  None,
];

const CONV_TABLE_01: [Option<Op32Instruction>; 8] = [
  Some(Op32Instruction::Sub),
  None,
  None,
  None,
  None,
  Some(Op32Instruction::ShiftArithmeticRight),
  None,
  None,
];

impl TryFrom<u32> for Op32Instruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    let funct7 = funct7!(inst);
    match funct7 {
      0b0000000 => CONV_TABLE_00[funct3 as usize].ok_or(()),
      0b0100000 => CONV_TABLE_01[funct3 as usize].ok_or(()),
      _ => Err(()),
    }
  }
}