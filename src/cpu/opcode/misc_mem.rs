use crate::funct3;

pub enum MiscMemInstruction {
  Fence,
}

impl MiscMemInstruction {
  pub fn exec(&self) {
    match self {
      MiscMemInstruction::Fence => (), // TODO: NOP for now, cuz I don't understand it
    }
  }
}

impl TryFrom<u32> for MiscMemInstruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    if funct3 == 0 {
      Ok(MiscMemInstruction::Fence)
    } else {
      Err(())
    }
  }
}
