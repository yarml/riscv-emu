use cpu::CPU;
use dev::{ReadMode, WriteMode};
use ram::RAM;
use rom::ROM;

mod bus;
mod cpu;
mod dev;
mod ram;
mod rom;
mod units;
mod utils;

pub fn shutup_unused() -> Result<(), ()> {
  let ram = RAM::new(16 * 1024 * 1024);
  let firmware =
    ROM::from_file("firmware.bin", true).expect("Could not load formware");
  let mut cpu = CPU::new(1);
  cpu.bus.attach_dev(Box::new(ram));
  cpu.bus.attach_dev(Box::new(firmware));

  cpu.harts[0].reg_write(2, 4);
  cpu.harts[0].reg_read(2);

  cpu.bus.write(0x0, WriteMode::Byte(3))?;
  cpu.bus.write(0x0, WriteMode::HalfWord(3))?;
  cpu.bus.write(0x0, WriteMode::Word(3))?;
  cpu.bus.write(0x0, WriteMode::DoubleWord(3))?;

  cpu.bus.read(0x0, ReadMode::Byte)?;
  cpu.bus.read(0x0, ReadMode::HalfWord)?;
  cpu.bus.read(0x0, ReadMode::Word)?;
  cpu.bus.read(0x0, ReadMode::DoubleWord)?;
  cpu.bus.read(0x0, ReadMode::Instruction)?;

  Ok(())
}
