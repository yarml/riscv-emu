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
    (($inst & 0b111000000000000) >> 12) as usize
  };
}

#[macro_export]
macro_rules! rs1 {
  ($inst:expr) => {
    (($inst & 0b11111000000000000000) >> 15) as usize
  };
}

#[macro_export]
macro_rules! rs2 {
  ($inst:expr) => {
    (($inst & 0b1111100000000000000000000) >> 20) as usize
  };
}

#[macro_export]
macro_rules! imm_i {
  ($inst:expr) => {
    $crate::map_bits! {
      [u64 : $inst as u64];
      copy [30, 20] => 0;
      repeat 31 => [63, 11];
    }
  };
}

#[macro_export]
macro_rules! imm_s {
  ($inst:expr) => {
    $crate::map_bits! {
      [u64 : $inst as u64];
      copy [11, 7] => 0;
      copy [30, 25] => 5;
      repeat 31 => [63, 11];
    }
  };
}

#[macro_export]
macro_rules! imm_b {
  ($inst:expr) => {
    $crate::map_bits! {
      [u64 : $inst as u64];
      copy [11, 8] => 1;
      copy [30, 25] => 5;
      copy 7 => 11;
      repeat 31 => [63, 12];
    }
  };
}

#[macro_export]
macro_rules! imm_u {
  ($inst:expr) => {
    $crate::map_bits! {
      [u64 : $inst as u64];
      copy [30, 12] => 12;
      repeat 31 => [63, 31];
    }
  };
}

#[macro_export]
macro_rules! imm_j {
  ($inst:expr) => {
    $crate::map_bits! {
      [u64 : $inst as u64];
      copy [30, 21] => 1;
      copy 20 => 11;
      copy [19, 12] => 12;
      repeat 31 => [63, 20];
    }
  };
}
