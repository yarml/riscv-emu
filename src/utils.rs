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

// Dear reader, these tests may seem basic, and they are.
// But I am proud of them, these are the first tests I ever write :)
// Alongside my first ever Rust macros.
#[cfg(test)]
mod test {
  #[test]
  fn test_align_up() {
    assert_eq!(align_up!(10, 16), 16);
    assert_eq!(align_up!(17, 16), 32);
    assert_eq!(align_dn!(16, 16), 16);
    assert_eq!(align_up!(10, 1), 10);
  }

  #[test]
  fn test_align_dn() {
    assert_eq!(align_dn!(10, 16), 0);
    assert_eq!(align_dn!(17, 16), 16);
    assert_eq!(align_dn!(16, 16), 16);
    assert_eq!(align_up!(10, 1), 10);
  }

  #[test]
  fn test_byte_mask() {
    assert_eq!(byte_mask!(0), 0x00000000000000FF);
    assert_eq!(byte_mask!(1), 0x000000000000FF00);
    assert_eq!(byte_mask!(2), 0x0000000000FF0000);
    assert_eq!(byte_mask!(3), 0x00000000FF000000);
    assert_eq!(byte_mask!(4), 0x000000FF00000000);
    assert_eq!(byte_mask!(5), 0x0000FF0000000000);
    assert_eq!(byte_mask!(6), 0x00FF000000000000);
    assert_eq!(byte_mask!(7), 0xFF00000000000000);
  }

  #[test]
  fn test_hword_mask() {
    assert_eq!(hword_mask!(0), 0x000000000000FFFF);
    assert_eq!(hword_mask!(1), 0x00000000FFFF0000);
    assert_eq!(hword_mask!(2), 0x0000FFFF00000000);
    assert_eq!(hword_mask!(3), 0xFFFF000000000000);
  }

  #[test]
  fn test_word_mask() {
    assert_eq!(word_mask!(0), 0x00000000FFFFFFFF);
    assert_eq!(word_mask!(1), 0xFFFFFFFF00000000);
  }

  #[test]
  fn test_sel_byte() {
    assert_eq!(sel_byte!(0xAB12CD34EF567890, 0), 0x90);
    assert_eq!(sel_byte!(0xAB12CD34EF567890, 1), 0x78);
    assert_eq!(sel_byte!(0xAB12CD34EF567890, 2), 0x56);
    assert_eq!(sel_byte!(0xAB12CD34EF567890, 3), 0xEF);
    assert_eq!(sel_byte!(0xAB12CD34EF567890, 4), 0x34);
    assert_eq!(sel_byte!(0xAB12CD34EF567890, 5), 0xCD);
    assert_eq!(sel_byte!(0xAB12CD34EF567890, 6), 0x12);
    assert_eq!(sel_byte!(0xAB12CD34EF567890, 7), 0xAB);
  }

  #[test]
  fn test_sel_hword() {
    assert_eq!(sel_hword!(0xAB12CD34EF567890, 0), 0x7890);
    assert_eq!(sel_hword!(0xAB12CD34EF567890, 1), 0xEF56);
    assert_eq!(sel_hword!(0xAB12CD34EF567890, 2), 0xCD34);
    assert_eq!(sel_hword!(0xAB12CD34EF567890, 3), 0xAB12);
  }

  #[test]
  fn test_sel_word() {
    assert_eq!(sel_word!(0xAB12CD34EF567890, 0), 0xEF567890);
    assert_eq!(sel_word!(0xAB12CD34EF567890, 1), 0xAB12CD34);
  }

  #[test]
  fn test_rep_byte() {
    assert_eq!(rep_byte!(0xFE35DD2412ABCD34, 2, 0xDF), 0xFE35DD2412DFCD34);
    assert_eq!(rep_byte!(0xFE35DD2412ABCD34, 4, 0xDF), 0xFE35DDDF12ABCD34);
  }

  #[test]
  fn test_rep_hword() {
    assert_eq!(
      rep_hword!(0xFE35DD2412ABCD34, 2, 0xABDF),
      0xFE35ABDF12ABCD34
    );
    assert_eq!(rep_hword!(0xFE35DD2412ABCD34, 3, 0xDF), 0x00DFDD2412ABCD34);
  }

  #[test]
  fn test_rep_word() {
    assert_eq!(
      rep_word!(0xFE35DD2412ABCD34, 1, 0x4444ABDF),
      0x4444ABDF12ABCD34
    );
    assert_eq!(rep_word!(0xFE35DD2412ABCD34, 0, 0xDF), 0xFE35DD24000000DF);
  }

  #[test]
  fn test_check_alignment() {
    assert_eq!(check_alignment!(4096, 16), true);
    assert_eq!(check_alignment!(15, 4), false);
    assert_eq!(check_alignment!(16, 16), true);
    assert_eq!(check_alignment!(0, 4), true);
    assert_eq!(check_alignment!(15, 4), false);
  }
}
