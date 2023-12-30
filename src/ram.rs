use crate::{
  align_up,
  dev::{
    Device, DeviceInfo, DeviceType, ReadMode, ReadResult, WriteMode,
    WriteResult,
  },
  rep_byte, rep_hword, rep_word, sel_byte, sel_hword, sel_word,
};

pub struct RAM {
  len: u64,
  mem: Vec<u64>,
}

impl RAM {
  pub fn new(len: u64) -> Self {
    let aclen = align_up!(len, 4096);
    Self {
      len: aclen,
      mem: vec![0u64; aclen as usize / 8],
    }
  }
}

impl Device for RAM {
  fn stat(&self) -> DeviceInfo {
    DeviceInfo {
      dev_type: DeviceType::RAM,
      alignment: 4096,
      len: self.len,
    }
  }

  fn write(&mut self, offset: u64, mode: WriteMode) -> WriteResult {
    let org = self.mem[(offset / 8) as usize];
    let rep = match mode {
      WriteMode::Byte(data) => {
        rep_byte!(org, offset % 8, data)
      }
      WriteMode::HalfWord(data) => {
        rep_hword!(org, (offset % 8) / 2, data)
      }
      WriteMode::Word(data) => {
        rep_word!(org, (offset % 8) / 4, data)
      }
      WriteMode::DoubleWord(data) => data,
    };
    self.mem[(offset / 8) as usize] = rep;
    WriteResult::Ok(())
  }

  fn read(&mut self, offset: u64, mode: ReadMode) -> ReadResult {
    let org = self.mem[(offset / 8) as usize];
    let data = match mode {
      ReadMode::Byte => sel_byte!(org, offset % 8),
      ReadMode::HalfWord => sel_hword!(org, (offset % 8) / 2),
      ReadMode::Word => sel_word!(org, (offset % 8) / 4),
      ReadMode::DoubleWord => org,
      ReadMode::Instruction => sel_word!(org, (offset % 8) / 4),
    };
    ReadResult::Ok(data)
  }
}
