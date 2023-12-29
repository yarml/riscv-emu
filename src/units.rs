#[macro_export]
macro_rules! kib {
  ($n:expr) => {
    $n * 1024
  };
}

#[macro_export]
macro_rules! mib {
  ($n:expr) => {
    $n * 1024 * 1024
  };
}

#[macro_export]
macro_rules! gib {
  ($n:expr) => {
    $n * 1024 * 1024 * 1024
  };
}
