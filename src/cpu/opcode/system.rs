use crate::{funct12, funct3};

pub enum SystemInstruction {
  EnvCall,
  EnvBreak,
}

impl SystemInstruction {
  pub fn exec(&self) {
    // NOP: I don't understand what these do either.
    // They're maybe described in the privilieged manual
    // From what little I understand, I think we need to transfer execution to another point
    // And set the hart to supervisor level, but that's not implemented yet so can't do that.
  }
}

impl TryFrom<u32> for SystemInstruction {
  type Error = ();

  fn try_from(inst: u32) -> Result<Self, Self::Error> {
    let funct3 = funct3!(inst);
    let funct12 = funct12!(inst);

    if funct3 != 0 {
      // Not PRIV. didn't read the privilieged manual yet, but I think all instruction are of PRIV type, at least for now...
      return Err(());
    }

    match funct12 {
      0 => Ok(SystemInstruction::EnvCall),
      1 => Ok(SystemInstruction::EnvBreak),
      _ => Err(()),
    }
  }
}
