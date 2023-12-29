/**
 * Memory Map provided by this emulator:
 *  [0, 4K): Vacant (so that adress 0 is invalid always)
 *  [4K, 1M): Device tree followed by vacant memory
 *  [1M, 4G): Mostly vacant memory + some memory mapped IO, as specified by device tree
 *  [4Gib ...): Main memory followed by vacant memory
 *
 * The mapper guarentees that all memory regions will start at a 4K boundary in [1M, 4G), except
 * main memory which always starts at 4G.
 */
use crate::{
  align_up,
  dev::{Device, DeviceInfo, ReadMode, ReadResult, WriteMode, WriteResult},
  rep_byte, rep_hword, rep_word, sel_byte, sel_hword, sel_word,
};

pub struct MemoryMapper {
  devs: Vec<ManagedDevice>,
}

struct ManagedDevice {
  adr_start: u64,
  adr_end: u64,
  dev: Box<dyn Device>,
}

impl MemoryMapper {
  pub fn new() -> Self {
    Self { devs: Vec::new() }
  }

  pub fn attach_dev(&mut self, dev: Box<dyn Device>) {
    let stat = dev.stat();
    let target_adr = match stat.name {
      "main_mem" => 4 * 1024 * 1024 * 1024,
      _ => {
        let highest_adr = self
          .devs
          .iter()
          .map(|dev| dev.adr_end)
          .fold(u64::MIN, |a, b| a.max(b));
        align_up!(highest_adr, 4096)
      }
    };

    self.devs.push(ManagedDevice {
      adr_start: target_adr,
      adr_end: target_adr + stat.len,
      dev,
    });
  }

  fn find_dev_in_range(
    &mut self,
    address: u64,
  ) -> Option<(u64, &mut ManagedDevice)> {
    let target_dev = self
      .devs
      .iter_mut()
      .find(|dev| dev.adr_start <= address && address < dev.adr_end);
    if let None = target_dev {
      return None;
    }
    let target_dev = target_dev.unwrap();

    let offset = target_dev.adr_start - address;
    Some((offset, target_dev))
  }

  pub fn write(&mut self, address: u64, mode: WriteMode) -> WriteResult {
    let dev = self.find_dev_in_range(address);
    if let None = dev {
      return WriteResult::Fail;
    }

    let (offset, dev) = dev.unwrap();
    dev.dev.write(offset, mode)
  }

  pub fn read(&mut self, address: u64, mode: ReadMode) -> ReadResult {
    let dev = self.find_dev_in_range(address);
    if let None = dev {
      return ReadResult::Fail;
    }

    let (offset, dev) = dev.unwrap();
    dev.dev.read(offset, mode)
  }
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

impl Device for MainMemory {
  fn stat(&self) -> DeviceInfo {
    DeviceInfo {
      name: "main_mem",
      len: self.len,
    }
  }

  fn write(&mut self, offset: u64, mode: WriteMode) -> WriteResult {
    if !mode.verify_alignment(offset) {
      return WriteResult::Fail;
    }
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
    WriteResult::Ok
  }

  fn read(&mut self, offset: u64, mode: ReadMode) -> ReadResult {
    if !mode.verify_alignment(offset) {
      return ReadResult::Fail;
    }
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
