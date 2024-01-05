use std::num::Wrapping;

use crate::{funct3, funct7, map_bits};

#[derive(Clone, Copy)]
pub enum Op32Instruction {
  Add,
  Sub,
  ShiftLogicalLeft,
  ShiftLogicalRight,
  ShiftArithmeticRight,
}

impl Op32Instruction {
  pub fn calc(
    &self,
    rs1v_u: Wrapping<u64>,
    rs2v_u: Wrapping<u64>,
  ) -> Wrapping<u64> {
    let rs1v = Wrapping(rs1v_u.0 as i64);
    let shamt_u = map_bits! {
      [usize : rs2v_u.0 as usize];
      copy [3, 0] => 0;
    };

    let result = match self {
      Op32Instruction::Add => rs1v_u + rs2v_u,
      Op32Instruction::Sub => rs1v_u - rs2v_u,
      Op32Instruction::ShiftLogicalLeft => rs1v_u << shamt_u,
      Op32Instruction::ShiftLogicalRight => rs1v_u >> shamt_u,
      Op32Instruction::ShiftArithmeticRight => {
        Wrapping((rs1v >> shamt_u).0 as u64)
      }
    };

    Wrapping(map_bits! {
      [u64 : result.0];
      copy [30, 0] => 0;
      repeat 31 => [63, 31];
    })
  }
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
