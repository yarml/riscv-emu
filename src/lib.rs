use cpu::Hart;
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

pub fn shutup_unused() {
  let ram = RAM::new(16 * 1024 * 1024);
  let firmware =
    ROM::from_file("firmware.bin", true).expect("Could not load formware");
  let mut hart = Hart::new();
  hart.bus.attach_dev(Box::new(ram));
  hart.bus.attach_dev(Box::new(firmware));

  hart.reg_write(2, 4);
  hart.reg_read(2);

  hart.bus.write(0x0, WriteMode::Byte(3));
  hart.bus.write(0x0, WriteMode::HalfWord(3));
  hart.bus.write(0x0, WriteMode::Word(3));
  hart.bus.write(0x0, WriteMode::DoubleWord(3));

  hart.bus.read(0x0, ReadMode::Byte);
  hart.bus.read(0x0, ReadMode::HalfWord);
  hart.bus.read(0x0, ReadMode::Word);
  hart.bus.read(0x0, ReadMode::DoubleWord);
  hart.bus.read(0x0, ReadMode::Instruction);
}
