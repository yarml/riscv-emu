#[macro_export]
macro_rules! align_up {
  ($org:expr, $alignment:expr) => {
    ($org + $alignment - 1) & !($alignment - 1)
  };
}

#[macro_export]
macro_rules! align_dn {
  ($org:expr, $alignment:expr) => {
    $org & !($alignment - 1)
  };
}

#[macro_export]
macro_rules! byte_mask {
  ($byte_idx:expr) => {
    (0xFFu64 << ($byte_idx * 8)) as u64
  };
}

#[macro_export]
macro_rules! hword_mask {
  ($hword_idx:expr) => {
    (0xFFFFu64 << ($hword_idx * 16)) as u64
  };
}

#[macro_export]
macro_rules! word_mask {
  ($word_idx:expr) => {
    (0xFFFFFFFFu64 << ($word_idx * 32)) as u64
  };
}

#[macro_export]
macro_rules! sel_byte {
  ($org:expr, $byte_idx:expr) => {
    (($org as u64 >> ($byte_idx * 8)) & 0xFF) as u64
  };
}

#[macro_export]
macro_rules! sel_hword {
  ($org:expr, $hword_idx:expr) => {
    (($org as u64 >> ($hword_idx * 16)) & 0xFFFF) as u64
  };
}

#[macro_export]
macro_rules! sel_word {
  ($org:expr, $word_idx:expr) => {
    (($org as u64 >> ($word_idx * 32)) & 0xFFFFFFFFu64) as u64
  };
}

#[macro_export]
macro_rules! rep_byte {
  ($org:expr, $byte_idx:expr, $replacement:expr) => {
    ($org & !$crate::byte_mask!($byte_idx))
      | ((($replacement as u64 & 0xFFu64) as u64) << (($byte_idx * 8) as u64))
        as u64
  };
}

#[macro_export]
macro_rules! rep_hword {
  ($org:expr, $hword_idx:expr, $replacement:expr) => {
    ($org & !$crate::hword_mask!($hword_idx))
      | ((($replacement as u64 & 0xFFFFu64) as u64)
        << (($hword_idx * 16) as u64)) as u64
  };
}

#[macro_export]
macro_rules! rep_word {
  ($org:expr, $word_idxord_idx:expr, $replacement:expr) => {
    ($org & !$crate::word_mask!($word_idxord_idx))
      | ((($replacement as u64 & 0xFFFFFFFFu64) as u64)
        << (($word_idxord_idx * 32) as u64)) as u64
  };
}

#[macro_export]
macro_rules! check_alignment {
  ($e:expr, $alignment:expr) => {
    $e % $alignment != 0
  };
}

#[macro_export]
macro_rules! sign_extend {
  ($org:expr, $bit_idx:expr) => {

  };
}
