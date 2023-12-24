use cpu::Hart;

mod cpu;
mod io;
mod mem;
mod utils;

pub fn shutup_unused() {
  let hart = Hart::new();
}
