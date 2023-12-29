use cpu::Hart;
use dev::{ReadMode, WriteMode};
use mem::MainMemory;

mod cpu;
mod dev;
mod mem;
mod utils;

pub fn shutup_unused() {
  let main_mem = MainMemory::new(16 * 1024 * 1024);
  let mut hart = Hart::new();
  hart.mapper.attach_dev(Box::new(main_mem));

  hart.reg_write(2, 4);
  hart.reg_read(2);

  hart.mapper.write(0x0, WriteMode::Byte(3));
  hart.mapper.write(0x0, WriteMode::HalfWord(3));
  hart.mapper.write(0x0, WriteMode::Word(3));
  hart.mapper.write(0x0, WriteMode::DoubleWord(3));

  hart.mapper.read(0x0, ReadMode::Byte);
  hart.mapper.read(0x0, ReadMode::HalfWord);
  hart.mapper.read(0x0, ReadMode::Word);
  hart.mapper.read(0x0, ReadMode::DoubleWord);
  hart.mapper.read(0x0, ReadMode::Instruction);
}
