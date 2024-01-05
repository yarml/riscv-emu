// Align $org to $alignment boundary.
// $alignment must be a power of 2.
#[macro_export]
macro_rules! align_up {
  ($org:expr, $alignment:expr) => {
    ($org + $alignment - 1) & !($alignment - 1)
  };
}

// Align $org to $alignment boundary going doing.
// $alignment must be a power of 2.
#[macro_export]
macro_rules! align_dn {
  ($org:expr, $alignment:expr) => {
    $org & !($alignment - 1)
  };
}

// Returns a u64 mask with 1s only in the selected byte index.
// Indices go from 0 to 7
#[macro_export]
macro_rules! byte_mask {
  ($byte_idx:expr) => {
    (0xFFu64 << ($byte_idx * 8)) as u64
  };
}

// Returns a u64 mask with 1s only in the selected half word index.
// Indices go from 0 to 3
#[macro_export]
macro_rules! hword_mask {
  ($hword_idx:expr) => {
    (0xFFFFu64 << ($hword_idx * 16)) as u64
  };
}

// Returns a u64 mask with 1s only in the selected word index.
// Indices go from 0 to 1
#[macro_export]
macro_rules! word_mask {
  ($word_idx:expr) => {
    (0xFFFFFFFFu64 << ($word_idx * 32)) as u64
  };
}

// Returns the selected byte from a u64.
// Indices go from 0 to 7
#[macro_export]
macro_rules! sel_byte {
  ($org:expr, $byte_idx:expr) => {
    (($org as u64 >> ($byte_idx * 8)) & 0xFF) as u64
  };
}

// Returns the selected half word from a u64.
// Indices go from 0 to 3
#[macro_export]
macro_rules! sel_hword {
  ($org:expr, $hword_idx:expr) => {
    (($org as u64 >> ($hword_idx * 16)) & 0xFFFF) as u64
  };
}

// Returns the selected word from a u64.
// Indices go from 0 to 1
#[macro_export]
macro_rules! sel_word {
  ($org:expr, $word_idx:expr) => {
    (($org as u64 >> ($word_idx * 32)) & 0xFFFFFFFFu64) as u64
  };
}

// Replaces a byte in a u64.
// Indices for from 0 to 7
#[macro_export]
macro_rules! rep_byte {
  ($org:expr, $byte_idx:expr, $replacement:expr) => {
    ($org & !$crate::byte_mask!($byte_idx))
      | ((($replacement as u64 & 0xFFu64) as u64) << (($byte_idx * 8) as u64))
        as u64
  };
}

// Replaces a half word in a u64.
// Indices for from 0 to 3
#[macro_export]
macro_rules! rep_hword {
  ($org:expr, $hword_idx:expr, $replacement:expr) => {
    ($org & !$crate::hword_mask!($hword_idx))
      | ((($replacement as u64 & 0xFFFFu64) as u64)
        << (($hword_idx * 16) as u64)) as u64
  };
}

// Replaces a word in a u64.
// Indices for from 0 to 1
#[macro_export]
macro_rules! rep_word {
  ($org:expr, $word_idxord_idx:expr, $replacement:expr) => {
    ($org & !$crate::word_mask!($word_idxord_idx))
      | ((($replacement as u64 & 0xFFFFFFFFu64) as u64)
        << (($word_idxord_idx * 32) as u64)) as u64
  };
}

// Returns true if $e is aligned to $alignment
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
macro_rules! select_bits {
  ($type:ty; $source:expr; $([$end:expr, $start:expr]),+) => {
    (0 as $type) $(| (!(<$type>::MAX << ($end - $start + 1)) << $start) & $source)*
  };
}

#[macro_export]
macro_rules! map_bits {
  ([$type:ty : $source:expr];) => {
    0 as $type
  };
  {[$type:ty : $source:expr]; repeat $src:expr => [$dest_end:expr, $dest_start:expr]; $($tail:tt)*} => {
    (!(<$type>::MAX << ($dest_end - $dest_start + 1)) << $dest_start) * (($source >> $src) & 1) | $crate::map_bits! {
      [$type : $source];
      $($tail)*
    }
  };
  {[$type:ty : $source:expr]; copy [$src_end:expr,$src_start:expr] => $dest:expr; $($tail:tt)*} => {
    (
      if $src_start > $dest {
        (((!(<$type>::MAX << ($src_end - $src_start + 1))) << $src_start) & $source) >> ($src_start - $dest)
      } else {
        (((!(<$type>::MAX << ($src_end - $src_start + 1))) << $src_start) & $source) << ($dest - $src_start)
      }| $crate::map_bits! {
        [$type : $source];
        $($tail)*
      }
    )
  };
  {[$type:ty : $source:expr]; copy $src:expr => $dest:expr; $($tail:tt)*} => {
    $crate::map_bits! {
      [$type : $source];
      copy [$src, $src] => $dest;
      $($tail)*
    }
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test_select_bits() {
    let result = select_bits!(u64; u64::MAX; [30, 24], [20, 10], [5, 0]);
    let expected =
      0b0000000000000000000000000000000001111111000111111111110000111111u64;
    assert_eq!(result, expected);

    let result = select_bits!(u32; u32::MAX; [30, 5], [2, 0]);
    let expected = 0b01111111111111111111111111100111u32;
    assert_eq!(result, expected);

    let source = 0b10100010010101001010010101010101u32;
    let result = select_bits!(u32; source; [29, 27], [10, 6]);
    let expected = 0b00100000000000000000010101000000u32;
    assert_eq!(result, expected);
  }

  #[test]
  fn test_map_bits() {
    let source = 0b10010010010101100001001111010101u32;
    let result = map_bits! {
      [u32 : source];
      repeat 28 => [15, 5];
    };
    let expected = 0b00000000000000001111111111100000u32;
    assert_eq!(result, expected);

    let result = map_bits! {
      [u32 : source];
      copy 20 => 0;
    };
    let expected = 0b00000000000000000000000000000001u32;
    assert_eq!(result, expected);

    let result = map_bits! {
      [u32 : source];
      copy [24, 21] => 1;
    };
    let expected = 0b00000000000000000000000000000100u32;
    assert_eq!(result, expected);

    let result = map_bits! {
      [u32 : source];
      copy 20 => 0;
      copy [24, 21] => 1;
      copy [30, 25] => 5;
      repeat 31 => [31, 11];
    };
    let expected = 0b11111111111111111111100100100101u32;
    assert_eq!(result, expected);
  }
}
