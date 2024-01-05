use std::num::Wrapping;

use crate::{funct3, map_bits};

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

impl OpImmInstruction {
  pub fn calc(
    &self,
    imm_u: Wrapping<u64>,
    rs1v_u: Wrapping<u64>,
  ) -> Wrapping<u64> {
    let rs1v = Wrapping(rs1v_u.0 as i64);
    let imm = Wrapping(imm_u.0 as i64);
    let shamt_u = map_bits! {
      [usize : imm_u.0 as usize];
      copy [4, 0] => 0;
    };

    match self {
      OpImmInstruction::Add => Wrapping((rs1v + imm).0 as u64),
      OpImmInstruction::ShiftLogicalLeft => rs1v_u << shamt_u,
      OpImmInstruction::ShiftLogicalRight => rs1v_u >> shamt_u,
      OpImmInstruction::ShiftArithmeticRight => {
        Wrapping((rs1v >> shamt_u).0 as u64)
      }
      OpImmInstruction::SetLessThan => {
        if rs1v < imm {
          Wrapping(1)
        } else {
          Wrapping(0)
        }
      }
      OpImmInstruction::SetLessThanUnsigned => {
        if rs1v_u < imm_u {
          Wrapping(1)
        } else {
          Wrapping(0)
        }
      }
      OpImmInstruction::XOR => rs1v_u ^ imm_u,
      OpImmInstruction::OR => rs1v_u | imm_u,
      OpImmInstruction::AND => rs1v_u & imm_u,
    }
  }
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

impl TryFrom<u32> for OpImmInstruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    let imm_6_11 = (inst & 0xFC000000) >> 26;
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
