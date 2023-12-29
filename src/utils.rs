#[macro_export]
macro_rules! align_up {
  ($n:expr, $b:expr) => {
    ($n + $b - 1) & !($b - 1)
  };
}

#[macro_export]
macro_rules! align_dn {
  ($n:expr, $b:expr) => {
    $n & !($b - 1)
  };
}

#[macro_export]
macro_rules! byte_mask {
  ($b:expr) => {
    (0xFFu64 << ($b * 8)) as u64
  };
}

#[macro_export]
macro_rules! hword_mask {
  ($hw:expr) => {
    (0xFFFFu64 << ($hw * 16)) as u64
  };
}

#[macro_export]
macro_rules! word_mask {
  ($w:expr) => {
    (0xFFFFFFFFu64 << ($w * 32)) as u64
  };
}

#[macro_export]
macro_rules! sel_byte {
  ($n:expr, $b:expr) => {
    (($n as u64 >> ($b * 8)) & 0xFF) as u64
  };
}

#[macro_export]
macro_rules! sel_hword {
  ($n:expr, $b:expr) => {
    (($n as u64 >> ($b * 16)) & 0xFFFF) as u64
  };
}

#[macro_export]
macro_rules! sel_word {
  ($n:expr, $w:expr) => {
    (($n as u64 >> ($w * 32)) & 0xFFFFFFFFu64) as u64
  };
}

#[macro_export]
macro_rules! rep_byte {
  ($n:expr, $b:expr, $r:expr) => {
    ($n & !$crate::byte_mask!($b))
      | ((($r as u64 & 0xFFu64) as u64) << (($b * 8) as u64)) as u64
  };
}

#[macro_export]
macro_rules! rep_hword {
  ($n:expr, $hw:expr, $r:expr) => {
    ($n & !$crate::hword_mask!($hw))
      | ((($r as u64 & 0xFFFFu64) as u64) << (($hw * 16) as u64)) as u64
  };
}

#[macro_export]
macro_rules! rep_word {
  ($n:expr, $w:expr, $r:expr) => {
    ($n & !$crate::word_mask!($w))
      | ((($r as u64 & 0xFFFFFFFFu64) as u64) << (($w * 32) as u64)) as u64
  };
}

#[macro_export]
macro_rules! check_alignment {
  ($e:expr, $a:expr) => {
    $e % $a != 0
  };
}
