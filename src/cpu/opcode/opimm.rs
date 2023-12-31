use std::num::Wrapping;

use crate::{cpu::hart::Hart, funct3, rd, rs1};

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
  pub fn exec(&self, inst: u32, hart: &mut Hart) {
    let rd = rd!(inst);
    let rs1 = rs1!(inst);
    let imm = Wrapping(
      ((inst & 0xFFF00000) >> 20) as u64 as i64
        | ((0xFFFFFFFFFFFFF000u64 as i64) * (inst >> 31) as i64),
    );
    let imm_u = Wrapping(imm.0 as u64);

    let s1u = hart.reg_read(rs1 as usize);
    let s1 = Wrapping(s1u.0 as i64);

    let shamt_u = (imm_u.0 & 0b111111) as usize;

    let result = match self {
      OpImmInstruction::Add => Wrapping((s1 + imm).0 as u64),
      OpImmInstruction::ShiftLogicalLeft => s1u << shamt_u,
      OpImmInstruction::ShiftLogicalRight => s1u >> shamt_u,
      OpImmInstruction::ShiftArithmeticRight => {
        Wrapping((s1 >> shamt_u).0 as u64)
      }
      OpImmInstruction::SetLessThan => {
        if s1 < imm {
          Wrapping(1)
        } else {
          Wrapping(0)
        }
      }
      OpImmInstruction::SetLessThanUnsigned => {
        if s1u < imm_u {
          Wrapping(1)
        } else {
          Wrapping(0)
        }
      }
      OpImmInstruction::XOR => s1u ^ imm_u,
      OpImmInstruction::OR => s1u | imm_u,
      OpImmInstruction::AND => s1u & imm_u,
    };

    hart.reg_write(rd as usize, result);
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
