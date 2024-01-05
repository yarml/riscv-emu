use std::num::Wrapping;

use crate::{funct3, funct7, map_bits};

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
  pub fn calc(
    &self,
    rs1v_u: Wrapping<u64>,
    rs2v_u: Wrapping<u64>,
  ) -> Wrapping<u64> {
    let rs1v = Wrapping(rs1v_u.0 as i64);
    let rs2v = Wrapping(rs1v_u.0 as i64);

    let shamt_u = map_bits! {
      [usize : rs2v_u.0 as usize];
      copy [4, 0] => 0;
    };

    match self {
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
    }
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
