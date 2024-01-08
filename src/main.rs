use std::num::Wrapping;

use riscv_emu::{cpu::CPU, ram::RAM, rom::ROM};

fn main() {
  let ram = RAM::new(64 * 1024 * 1024);
  let firmware =
    ROM::from_file("firmware.bin", true).expect("Could not load firmware");
  let mut cpu = CPU::new(1);
  cpu.bus.attach_dev(Box::new(ram));
  let firmware_start = cpu.bus.attach_dev(Box::new(firmware));

  // Reset CPU, just making sure
  cpu.harts.iter_mut().for_each(|hart| {
    (0..32).for_each(|regn| hart.reg_write(regn, Wrapping(0)));
    hart.pc = Wrapping(firmware_start);
    hart.print_state();
  });

  eprintln!("Begin");

  loop {
    cpu.cycle();
  }
}
