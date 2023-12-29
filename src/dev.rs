use crate::check_alignment;

pub struct DeviceInfo {
  pub dev_type: DeviceType,
  pub alignment: u64,
  pub len: u64,
}

pub enum DeviceType {
  RAM,
  ROM,
  MemoryTable,
  Generic,
}

pub trait Device {
  fn stat(&self) -> DeviceInfo;

  fn write(&mut self, offset: u64, mode: WriteMode) -> WriteResult;
  fn read(&mut self, offset: u64, mode: ReadMode) -> ReadResult;
}

pub enum ReadMode {
  Byte,
  HalfWord,
  Word,
  DoubleWord,
  Instruction,
}

pub enum ReadResult {
  Fail,
  Ok(u64),
}

pub enum WriteMode {
  Byte(u8),
  HalfWord(u16),
  Word(u32),
  DoubleWord(u64),
}

pub enum WriteResult {
  Fail,
  Ok,
}

impl ReadMode {
  pub fn verify_alignment(&self, offset: u64) -> bool {
    check_alignment!(offset, self.alignment())
  }

  pub fn alignment(&self) -> u64 {
    match self {
      ReadMode::Byte => 1,
      ReadMode::HalfWord => 2,
      ReadMode::Word => 4,
      ReadMode::DoubleWord => 8,
      ReadMode::Instruction => 4,
    }
  }
}

impl WriteMode {
  pub fn verify_alignment(&self, offset: u64) -> bool {
    check_alignment!(offset, self.alignment())
  }

  pub fn alignment(&self) -> u64 {
    match self {
      WriteMode::Byte(_) => 1,
      WriteMode::HalfWord(_) => 2,
      WriteMode::Word(_) => 4,
      WriteMode::DoubleWord(_) => 8,
    }
  }
}
