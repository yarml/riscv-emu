use crate::{align_up, io::IODevice, rep_byte, rep_hword, check_alignment, rep_word, sel_byte, sel_hword, sel_word};

pub struct MemoryMapper {
  io_devs: Vec<ManagedIODevice>,
}

struct AddressRegion {
  start: u64,
  len: u64,
}

enum ManagedIODevice {
  Unmapped(Box<dyn IODevice>),
  Mapped(AddressRegion, Box<dyn IODevice>),
}

impl MemoryMapper {
  pub fn new() -> Self {
    Self {
      io_devs: Vec::new(),
    }
  }

  pub fn attach_dev(&mut self, dev: Box<dyn IODevice>) {
    self.io_devs.push(ManagedIODevice::Unmapped(dev))
  }

  pub fn init(&mut self) {}
}

pub struct MainMemory {
  pub len: u64,
  pub mem: Vec<u64>,
}

impl MainMemory {
  pub fn new(len: u64) -> Self {
    let aclen = align_up!(len, 4096);
    Self {
      len: aclen,
      mem: vec![0u64; aclen as usize / 8],
    }
  }
}

impl IODevice for MainMemory {
  fn stat(&self) -> (&'static str, u64) {
    ("main_mem", self.len)
  }

  fn write_byte(&mut self, offset: usize, data: u8) {
    let org = self.mem[offset / 8];
    self.mem[offset / 8] = rep_byte!(org, offset % 8, data);
  }

  fn write_hword(&mut self, offset: usize, data: u16) {
    check_alignment!(offset, 2);
    let org = self.mem[offset / 8];
    self.mem[offset / 8] = rep_hword!(org, (offset % 8) / 2, data);

  }

  fn write_word(&mut self, offset: usize, data: u32) {
    check_alignment!(offset, 4);
    let org = self.mem[offset / 8];
    self.mem[offset / 8] = rep_word!(org, (offset % 8) / 4, data);
  }

  fn write_dword(&mut self, offset: usize, data: u64) {
    check_alignment!(offset, 8);
    self.mem[offset / 8] = data;
  }

  fn read_byte(&mut self, offset: usize) -> u8 {
    sel_byte!(self.mem[offset / 8], offset % 8)
  }

  fn read_hword(&mut self, offset: usize) -> u16 {
    check_alignment!(offset, 2);
    sel_hword!(self.mem[offset / 8], (offset % 8) / 2)
  }

  fn read_word(&mut self, offset: usize) -> u32 {
    check_alignment!(offset, 4);
    sel_word!(self.mem[offset / 8], (offset % 8) / 4)
  }

  fn read_dword(&mut self, offset: usize) -> u64 {
    check_alignment!(offset, 8);
    self.mem[offset / 8]
  }
}
