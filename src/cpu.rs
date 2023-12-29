use crate::bus::Bus;

pub struct Hart {
  regfile: [u64; 32], // Access through reg_read & reg_write
  pub pc: u64,
  pub bus: Bus,
}

impl Hart {
  pub fn new() -> Self {
    Self {
      regfile: [0u64; 32],
      pc: 0,
      bus: Bus::new(),
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
}
