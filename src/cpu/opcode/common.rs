#[macro_export]
macro_rules! funct3 {
  ($inst:expr) => {
    ($inst & 0b111000000000000) >> 12
  };
}

#[macro_export]
macro_rules! funct7 {
  ($inst:expr) => {
    ($inst & 0xFE000000) >> 25
  };
}

#[macro_export]
macro_rules! rd {
  ($inst:expr) => {
    ($inst & 0b111000000000000) >> 12
  };
}

#[macro_export]
macro_rules! rs1 {
  ($inst:expr) => {
    ($inst & 0b11111000000000000000) >> 15
  };
}

#[macro_export]
macro_rules! rs2 {
  ($inst:expr) => {
    ($inst & 0b1111100000000000000000000) >> 20
  };
}

#[macro_export]
macro_rules! imm_i {
  ($inst:expr) => {
    
  };
}
