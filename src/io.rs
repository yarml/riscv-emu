pub trait IODevice {
  fn stat(&self) -> (&'static str, u64);

  fn write_byte(&mut self, offset: usize, data: u8);
  fn write_hword(&mut self, offset: usize, data: u16);
  fn write_word(&mut self, offset: usize, data: u32);
  fn write_dword(&mut self, offset: usize, data: u64);

  fn read_byte(&mut self, offset: usize) -> u8;
  fn read_hword(&mut self, offset: usize) -> u16;
  fn read_word(&mut self, offset: usize) -> u32;
  fn read_dword(&mut self, offset: usize) -> u64;
}
