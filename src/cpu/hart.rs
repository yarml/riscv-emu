use crate::{bus::Bus, dev::ReadMode};

use super::{cycle::CycleResult, opcode::Opcode};

pub struct Hart {
  regfile: [u64; 32], // Access through reg_read & reg_write
  pub pc: u64,
}

impl Hart {
  pub fn new() -> Self {
    Hart {
      regfile: [0u64; 32],
      pc: 0,
    }
  }

  #[inline]
  pub fn reg_read(&self, regn: usize) -> u64 {
    if regn == 0 {
      0
    } else {
      self.regfile[regn]
    }
  }

  #[inline]
  pub fn reg_write(&mut self, regn: usize, data: u64) {
    if regn != 0 {
      self.regfile[regn] = data;
    }
  }

  pub fn cycle(&mut self, bus: &mut Bus) -> CycleResult {
    let inst = bus.read(self.pc, ReadMode::Instruction)? as u32;
    let opcode: Opcode = (inst & 0b1111111).try_into()?;

    CycleResult::Ok(())
  }
}
