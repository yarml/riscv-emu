use std::num::Wrapping;

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

  fn write(&mut self, offset: Wrapping<u64>, mode: WriteMode) -> WriteResult;
  fn read(&mut self, offset: Wrapping<u64>, mode: ReadMode) -> ReadResult;
}

pub enum ReadMode {
  Byte,
  HalfWord,
  Word,
  DoubleWord,
  Instruction,
}

pub type ReadResult = Result<u64, ()>;

pub enum WriteMode {
  Byte(Wrapping<u8>),
  HalfWord(Wrapping<u16>),
  Word(Wrapping<u32>),
  DoubleWord(Wrapping<u64>),
}

pub type WriteResult = Result<(), ()>;
pub type AlignmentResult = Result<(), ()>;

impl ReadMode {
  pub fn verify_alignment(&self, offset: Wrapping<u64>) -> AlignmentResult {
    check_alignment!(offset.0, self.alignment())
      .then_some(())
      .ok_or(())
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
  pub fn verify_alignment(&self, offset: Wrapping<u64>) -> bool {
    check_alignment!(offset.0, self.alignment())
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
