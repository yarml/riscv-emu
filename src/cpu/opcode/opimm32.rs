use crate::funct3;

#[derive(Clone, Copy)]
pub enum OpImm32Instruction {
  Add,
  ShiftLogicalLeft,
  ShiftLogicalRight,
  ShiftArithmeticRight,
}

// None means check CONV_TABLE2_*
const CONV_TABLE: [Option<OpImm32Instruction>; 8] = [
  Some(OpImm32Instruction::Add),
  Some(OpImm32Instruction::ShiftLogicalLeft),
  None,
  None,
  None,
  None,
  None,
  None,
];

const CONV_TABLE2_00: [Option<OpImm32Instruction>; 8] = [
  None,
  None,
  None,
  None,
  None,
  Some(OpImm32Instruction::ShiftLogicalRight),
  None,
  None,
];
const CONV_TABLE2_01: [Option<OpImm32Instruction>; 8] = [
  None,
  None,
  None,
  None,
  None,
  Some(OpImm32Instruction::ShiftArithmeticRight),
  None,
  None,
];

impl TryFrom<u32> for OpImm32Instruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    let imm_5_11 = (inst & 0xFC000000) >> 26;
    match CONV_TABLE[funct3 as usize] {
      Some(inst) => Ok(inst),
      None => {
        if imm_5_11 == 0 {
          CONV_TABLE2_00[funct3 as usize].ok_or(())
        } else if imm_5_11 == 0b0100000 {
          CONV_TABLE2_01[funct3 as usize].ok_or(())
        } else {
          Err(())
        }
      }
    }
  }
}
