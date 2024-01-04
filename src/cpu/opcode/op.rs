use std::num::Wrapping;

use crate::{cpu::hart::Hart, funct3, funct7, rd, rs1, rs2};

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

impl OpInstruction {
  pub fn exec(&self, inst: u32, hart: &mut Hart) {
    let rd = rd!(inst);
    let rs1 = rs1!(inst);
    let rs2 = rs2!(inst);

    let rs1v_u = hart.reg_read(rs1);
    let rs2v_u = hart.reg_read(rs2);

    let rs1v = Wrapping(rs1v_u.0 as i64);
    let rs2v = Wrapping(rs1v_u.0 as i64);

    let shamt_u = (rs2v_u.0 & 0b11111) as usize;

    let result = match self {
      OpInstruction::Add => rs1v_u + rs2v_u,
      OpInstruction::Sub => rs1v_u - rs2v_u,
      OpInstruction::ShiftLogicalLeft => rs1v_u << shamt_u,
      OpInstruction::ShiftLogicalRight => rs1v_u >> shamt_u,
      OpInstruction::ShiftArithmeticRight => {
        Wrapping((rs1v >> shamt_u).0 as u64)
      }
      OpInstruction::SetLessThan => {
        if rs1v < rs2v {
          Wrapping(1)
        } else {
          Wrapping(0)
        }
      }
      OpInstruction::SetLessThanUnsigned => {
        if rs1v_u < rs2v_u {
          Wrapping(1)
        } else {
          Wrapping(0)
        }
      }
      OpInstruction::XOR => rs1v_u ^ rs2v_u,
      OpInstruction::OR => rs1v_u | rs2v_u,
      OpInstruction::AND => rs1v_u & rs2v_u,
    };

    hart.reg_write(rd, result);
  }
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

impl TryFrom<u32> for OpInstruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    let funct7 = funct7!(inst);
    match funct7 {
      0b0000000 => Ok(CONV_TABLE_00[funct3 as usize]),
      0b0100000 => CONV_TABLE_01[funct3 as usize].ok_or(()),
      _ => Err(()),
    }
  }
}
