use std::{
  fs::{self, File},
  io::{self, Read},
};

use byteorder::{ByteOrder, LittleEndian};

use crate::{
  dev::{Device, DeviceInfo, DeviceType, ReadMode, ReadResult, WriteResult},
  kib, sel_byte, sel_hword, sel_word,
};

pub struct ROM {
  len: u64,
  mem: Vec<u64>,
  allow_fetch: Result<(), ()>,
}

impl ROM {
  pub fn from_file(path: &str, allow_fetch: bool) -> io::Result<Self> {
    let mut f = File::open(path)?;
    let metadata = fs::metadata(path)?;

    let mut buffer = vec![0u8; metadata.len() as usize];

    let len = f.read(&mut buffer)?;
    buffer.truncate(len);

    // Push a few 0s until buffer.len() is divisible by 8
    while buffer.len() % 8 != 0 {
      buffer.push(0u8);
    }

    // Convert the Vec<u8> to Vec<u64>
    let len = buffer.len();
    let len64 = len / 8;

    let mut buffer64 = vec![0u64; len64];

    LittleEndian::read_u64_into(&buffer, &mut buffer64);

    Ok(Self {
      len: buffer.len() as u64,
      mem: buffer64,
      allow_fetch: allow_fetch.then_some(()).ok_or(()),
    })
  }
}

impl Device for ROM {
  fn stat(&self) -> crate::dev::DeviceInfo {
    DeviceInfo {
      dev_type: DeviceType::ROM,
      alignment: kib!(4),
      len: self.len,
    }
  }

  fn write(
    &mut self,
    _offset: u64,
    _mode: crate::dev::WriteMode,
  ) -> crate::dev::WriteResult {
    WriteResult::Err(())
  }

  fn read(
    &mut self,
    offset: u64,
    mode: crate::dev::ReadMode,
  ) -> crate::dev::ReadResult {
    let org = self.mem[(offset / 8) as usize];
    let data = match mode {
      ReadMode::Byte => sel_byte!(org, offset % 8),
      ReadMode::HalfWord => sel_hword!(org, (offset % 8) / 2),
      ReadMode::Word => sel_word!(org, (offset % 8) / 4),
      ReadMode::DoubleWord => org,
      ReadMode::Instruction => {
        self.allow_fetch?;
        sel_word!(org, (offset % 8) / 4)
      }
    };
    ReadResult::Ok(data)
  }
}
