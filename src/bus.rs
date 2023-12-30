/**
 * Memory Map provided by this emulator:
 *  [0, 4K): Vacant (so that adress 0 is invalid always)
 *  [4K, 16M): Memory table followed by vacant memory
 *  [16M, ...): Specified by memory table (Combination of main memoy, ROM, MMIO, vacant memory)
 */
use crate::{
  align_up,
  dev::{Device, ReadMode, ReadResult, WriteMode, WriteResult},
  mib,
};

pub struct Bus {
  devs: Vec<BusDevice>,
}

struct BusDevice {
  adr_start: u64,
  adr_end: u64, // Invalid address already
  dev: Box<dyn Device>,
}

impl Bus {
  pub fn new() -> Self {
    Self { devs: Vec::new() }
  }

  pub fn attach_dev(&mut self, dev: Box<dyn Device>) {
    let stat = dev.stat();
    let target_adr = {
      let highest_adr = self
        .devs
        .iter()
        .map(|dev| dev.adr_end)
        .fold(0, |a, b| a.max(b));
      align_up!(
        if highest_adr == 0 {
          mib!(16)
        } else {
          highest_adr
        },
        stat.alignment
      )
    };

    self.devs.push(BusDevice {
      adr_start: target_adr,
      adr_end: target_adr + stat.len,
      dev,
    });
  }

  fn find_dev_in_range(
    &mut self,
    address: u64,
  ) -> Option<(u64, &mut BusDevice)> {
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
    if !mode.verify_alignment(address) {
      return WriteResult::Err(());
    }

    let dev = self.find_dev_in_range(address);
    if let None = dev {
      return WriteResult::Err(());
    }

    let (offset, dev) = dev.unwrap();
    dev.dev.write(offset, mode)
  }

  pub fn read(&mut self, address: u64, mode: ReadMode) -> ReadResult {
    mode.verify_alignment(address)?;

    let (offset, dev) = self.find_dev_in_range(address).ok_or(())?;
    dev.dev.read(offset, mode)
  }
}
