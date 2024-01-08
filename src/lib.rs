use std::num::Wrapping;

use cpu::CPU;
use dev::{ReadMode, WriteMode};
use ram::RAM;
use rom::ROM;

pub mod bit;
pub mod bus;
pub mod cpu;
pub mod dev;
pub mod ram;
pub mod rom;
pub mod units;

pub fn shutup_unused() -> Result<(), ()> {
  let ram = RAM::new(16 * 1024 * 1024);
  let firmware =
    ROM::from_file("firmware.bin", true).expect("Could not load formware");
  let mut cpu = CPU::new(1);
  cpu.bus.attach_dev(Box::new(ram));
  cpu.bus.attach_dev(Box::new(firmware));

  cpu.harts[0].reg_write(2, Wrapping(4));
  cpu.harts[0].reg_read(2);

  cpu.bus.write(Wrapping(0x0), WriteMode::Byte(Wrapping(3)))?;
  cpu
    .bus
    .write(Wrapping(0x0), WriteMode::HalfWord(Wrapping(3)))?;
  cpu.bus.write(Wrapping(0x0), WriteMode::Word(Wrapping(3)))?;
  cpu
    .bus
    .write(Wrapping(0x0), WriteMode::DoubleWord(Wrapping(3)))?;

  cpu.bus.read(Wrapping(0x0), ReadMode::Byte)?;
  cpu.bus.read(Wrapping(0x0), ReadMode::HalfWord)?;
  cpu.bus.read(Wrapping(0x0), ReadMode::Word)?;
  cpu.bus.read(Wrapping(0x0), ReadMode::DoubleWord)?;
  cpu.bus.read(Wrapping(0x0), ReadMode::Instruction)?;

  cpu.cycle();

  Ok(())
}
