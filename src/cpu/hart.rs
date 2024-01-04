use std::num::Wrapping;

use crate::{bus::Bus, dev::ReadMode};

use super::{
  cycle::CycleResult,
  opcode::{Opcode, OpcodeExecResult},
};

pub struct Hart {
  regfile: [Wrapping<u64>; 32], // Access through reg_read & reg_write
  pub pc: Wrapping<u64>,
}

impl Hart {
  pub fn new() -> Self {
    Hart {
      regfile: [Wrapping(0u64); 32],
      pc: Wrapping(0u64),
    }
  }

  #[inline]
  pub fn reg_read(&self, regn: usize) -> Wrapping<u64> {
    if regn == 0 {
      Wrapping(0u64)
    } else {
      self.regfile[regn]
    }
  }

  #[inline]
  pub fn reg_write(&mut self, regn: usize, data: Wrapping<u64>) {
    if regn != 0 {
      self.regfile[regn] = data;
    }
  }

  pub fn cycle(&mut self, bus: &mut Bus) -> CycleResult {
    let inst = bus.read(self.pc, ReadMode::Instruction)? as u32;
    let opcode: Opcode = (inst & 0b1111111).try_into()?;

    let next_pc = match opcode.exec(inst, self) {
      OpcodeExecResult::Fail => todo!(),
      OpcodeExecResult::Normal => self.pc + Wrapping(4),
      OpcodeExecResult::SetPC(new_pc) => new_pc,
    };

    self.pc = next_pc;

    CycleResult::Ok(())
  }
}
