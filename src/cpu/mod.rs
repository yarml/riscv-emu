pub mod cycle;
pub mod hart;
pub mod opcode;

use self::hart::Hart;
use crate::bus::Bus;

pub struct CPU {
  pub harts: Vec<Hart>,
  pub bus: Bus,
}

impl CPU {
  pub fn new(num_harts: usize) -> Self {
    let mut harts = Vec::with_capacity(num_harts);

    for _ in 0..num_harts {
      harts.push(Hart::new());
    }

    Self {
      harts,
      bus: Bus::new(),
    }
  }

  pub fn cycle(&mut self) {
    self
      .harts
      .iter_mut()
      .for_each(|hart| match hart.cycle(&mut self.bus) {
        Err(_) => {
          hart.print_state();
          panic!("Hart failed to cycle");
        }
        _ => {}
      });
  }
}
