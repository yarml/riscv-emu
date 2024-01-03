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

// I am very much surprised I got this to work
// Me proud of myself
// Now I just gotta remember where I wanted to use it...
#[macro_export]
macro_rules! build_num {
  ($type:ty; $([$end:expr, $start:expr]),+) => {
    0 as $type $(| (!(<$type>::MAX << ($end - $start + 1))) << $start)*
  };
}

mod test {
  #[test]
  fn test_build_num() {
    let result = build_num!(u64; [30, 24], [20, 10], [5, 0]);
    let expected =
      0b0000000000000000000000000000000001111111000111111111110000111111u64;
    assert_eq!(result, expected);

    let result = build_num!(u32; [30, 5], [2, 0]);
    let expected = 0b01111111111111111111111111100111u32;
    assert_eq!(result, expected);
  }
}
