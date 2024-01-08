pub mod branch;
pub mod common;
pub mod load;
pub mod misc_mem;
pub mod op;
pub mod op32;
pub mod opimm;
pub mod opimm32;
pub mod store;
pub mod system;

use std::num::Wrapping;

use crate::{bus::Bus, imm_b, imm_i, imm_j, imm_s, imm_u, rd, rs1, rs2};

use self::{
  branch::BranchInstruction, load::LoadInstruction,
  misc_mem::MiscMemInstruction, op::OpInstruction, op32::Op32Instruction,
  opimm::OpImmInstruction, opimm32::OpImm32Instruction,
  store::StoreInstruction, system::SystemInstruction,
};
use super::hart::Hart;

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

pub enum OpcodeExecResult {
  DecodeFail,
  Fail, // Maybe add more detail here, idk
  Normal,
  RelPC(Wrapping<u64>),
  AbsPC(Wrapping<u64>),
}

impl Opcode {
  pub fn exec(
    &self,
    inst: u32,
    hart: &mut Hart,
    bus: &mut Bus,
  ) -> OpcodeExecResult {
    let rd = rd!(inst);
    let rs1 = rs1!(inst);
    let rs2 = rs2!(inst);

    let rs1v_u = hart.reg_read(rs1);
    let rs2v_u = hart.reg_read(rs2);

    let immi_u = Wrapping(imm_i!(inst));
    let immu_u = Wrapping(imm_u!(inst));
    let immj_u = Wrapping(imm_j!(inst));
    let immb_u = Wrapping(imm_b!(inst));
    let imms_u = Wrapping(imm_s!(inst));
    match self {
      Self::OpImm => match OpImmInstruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(opimm_inst) => {
          let result = opimm_inst.calc(immi_u, rs1v_u);
          hart.reg_write(rd, result);
          OpcodeExecResult::Normal
        }
      },
      Self::OpImm32 => match OpImm32Instruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(opimm32_inst) => {
          let result = opimm32_inst.calc(immi_u, rs1v_u);
          hart.reg_write(rd, result);
          OpcodeExecResult::Normal
        }
      },
      Self::Op => match OpInstruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(op_inst) => {
          let result = op_inst.calc(rs1v_u, rs2v_u);
          hart.reg_write(rd, result);
          OpcodeExecResult::Normal
        }
      },
      Self::Op32 => match Op32Instruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(op32_inst) => {
          let result = op32_inst.calc(rs1v_u, rs2v_u);
          hart.reg_write(rd, result);
          OpcodeExecResult::Normal
        }
      },
      Self::LUI => {
        hart.reg_write(rd, immu_u);
        OpcodeExecResult::Normal
      }
      Self::AUIPC => {
        hart.reg_write(rd, immu_u + hart.pc);
        OpcodeExecResult::Normal
      }
      Self::JAL => {
        hart.reg_write(rd, hart.next_pc());
        OpcodeExecResult::RelPC(immj_u)
      }
      Self::JALR => {
        hart.reg_write(rd, hart.next_pc());
        OpcodeExecResult::AbsPC(rs1v_u + immi_u)
      }
      Self::Branch => match BranchInstruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(branch_inst) => {
          if branch_inst.check(rs1v_u.0, rs2v_u.0) {
            OpcodeExecResult::RelPC(immb_u)
          } else {
            OpcodeExecResult::Normal
          }
        }
      },
      Self::Load => match LoadInstruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(load_inst) => match bus.read(rs1v_u + immi_u, load_inst.into()) {
          Err(_) => OpcodeExecResult::Fail,
          Ok(data) => {
            hart.reg_write(rd, Wrapping(load_inst.conv_loaded_data(data)));
            OpcodeExecResult::Normal
          }
        },
      },
      Self::Store => match StoreInstruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(store_inst) => {
          match bus.write(imms_u + rs1v_u, store_inst.into_write_mode(rs2v_u.0))
          {
            Err(_) => OpcodeExecResult::Fail,
            Ok(_) => OpcodeExecResult::Normal,
          }
        }
      },
      Self::MiscMem => match MiscMemInstruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(misc_mem_inst) => {
          misc_mem_inst.exec();
          OpcodeExecResult::Normal
        }
      },
      Self::System => match SystemInstruction::try_from(inst) {
        Err(_) => OpcodeExecResult::DecodeFail,
        Ok(sys_inst) => {
          // Currently SYSTEM opcode is not implemented
          // So I am using this to print hart debug info
          hart.print_state();

          sys_inst.exec();
          OpcodeExecResult::Normal
        }
      },
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
    let opcode_5_6 = (opcode & 0b1100000) >> 5;

    if opcode_0_1 != 0b11 {
      return Err(());
    }

    if opcode_2_4 == 0b111 {
      return Err(());
    }

    CONV_TABLE[opcode_5_6 as usize][opcode_2_4 as usize].ok_or(())
  }
}
