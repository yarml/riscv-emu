use crate::mem::MemoryMapper;

pub struct Hart {
  pub regfile: [u64; 32],
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

  pub fn init() {}
}
