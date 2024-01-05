use std::num::Wrapping;

use crate::{funct3, map_bits};

#[derive(Clone, Copy)]
pub enum OpImm32Instruction {
  Add,
  ShiftLogicalLeft,
  ShiftLogicalRight,
  ShiftArithmeticRight,
}

impl OpImm32Instruction {
  pub fn calc(
    &self,
    imm_u: Wrapping<u64>,
    rs1v_u: Wrapping<u64>,
  ) -> Wrapping<u64> {
    let rs1v = Wrapping(rs1v_u.0 as i64);
    let shamt_u = map_bits! {
      [usize : imm_u.0 as usize];
      copy [24, 20] => 0;
    };
    let result = match self {
      Self::Add => rs1v_u + imm_u,
      Self::ShiftLogicalLeft => rs1v_u << shamt_u,
      Self::ShiftLogicalRight => rs1v_u >> shamt_u,
      Self::ShiftArithmeticRight => Wrapping((rs1v >> shamt_u).0 as u64),
    };

    Wrapping(map_bits! {
      [u64 : result.0];
      copy [30, 0] => 0;
      repeat 31 => [63, 31];
    })
  }
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
