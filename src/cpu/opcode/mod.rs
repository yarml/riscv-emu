mod branch;
mod common;
mod load;
mod op;
mod op32;
mod opimm;
mod opimm32;
mod store;

use crate::{rd, rs1, rs2};

use self::opimm::OpImmInstruction;
use super::{cycle::CycleResult, hart::Hart};

#[derive(Clone, Copy)]
pub enum Opcode {
  OpImm,
  LUI,
  AUIPC,
  Op,
  JAL,
  JALR,
  Branch,
  Load,
  Store,
  MiscMem,
  System,
  OpImm32,
  Op32,
  // Add AMO when atomic memory operations are implemented
}

impl Opcode {
  pub fn exec(&self, inst: u32, hart: &mut Hart) -> CycleResult {
    let rd = rd!(inst);
    let rs1 = rs1!(inst);
    let rs2 = rs2!(inst);

    match self {
      Opcode::OpImm => Ok(OpImmInstruction::try_from(inst)?.exec(inst, hart)),
      Opcode::LUI => todo!(),
      Opcode::AUIPC => todo!(),
      Opcode::Op => todo!(),
      Opcode::JAL => todo!(),
      Opcode::JALR => todo!(),
      Opcode::Branch => todo!(),
      Opcode::Load => todo!(),
      Opcode::Store => todo!(),
      Opcode::MiscMem => todo!(),
      Opcode::System => todo!(),
      Opcode::OpImm32 => todo!(),
      Opcode::Op32 => todo!(),
    }
  }
}

const CONV_TABLE: [[Option<Opcode>; 7]; 4] = [
  [
    Some(Opcode::Load),
    None, // LOAD-FP
    None, // custom-0
    Some(Opcode::MiscMem),
    Some(Opcode::OpImm),
    Some(Opcode::AUIPC),
    Some(Opcode::OpImm32),
  ],
  [
    Some(Opcode::Store),
    None, // STORE-FP
    None, // custom-1
    None, // AMO
    Some(Opcode::Op),
    Some(Opcode::LUI),
    Some(Opcode::Op32),
  ],
  [
    None, // MADD
    None, // MSUB
    None, // NMSUB
    None, // NMADD
    None, // OP-FP
    None, // reserved
    None, // custom-2 / rv128
  ],
  [
    Some(Opcode::Branch),
    Some(Opcode::JALR),
    None, // reserved
    Some(Opcode::JAL),
    Some(Opcode::System),
    None, // resered
    None, // custom-3 / rv128
  ],
];

impl TryFrom<u32> for Opcode {
  type Error = ();

  fn try_from(opcode: u32) -> Result<Self, Self::Error> {
    let opcode_0_1 = opcode & 0b11;
    let opcode_2_4 = (opcode & 0b11100) >> 2;
    let opcode_5_6 = (opcode & 0x1100000) >> 5;

    if opcode_0_1 != 0b11 {
      return Err(());
    }

    if opcode_2_4 == 0b111 {
      return Err(());
    }

    CONV_TABLE[opcode_5_6 as usize][opcode_2_4 as usize].ok_or(())
  }
}