use crate::mem::MemoryMapper;

pub struct Hart {
  regfile: [u64; 32], // Access through reg_read & reg_write
  pub pc: u64,
  pub mapper: MemoryMapper,
}

impl Hart {
  pub fn new() -> Self {
    Self {
      regfile: [0u64; 32],
      pc: 0,
      mapper: MemoryMapper::new(),
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
