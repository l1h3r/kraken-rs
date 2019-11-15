#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum Interval {
  M1 = 1,
  M5 = 5,
  M15 = 15,
  M30 = 30,
  H1 = 60,
  H4 = 240,
  D1 = 1440,
  D7 = 10080,
  D15 = 21600,
}

impl Default for Interval {
  fn default() -> Self {
    Interval::M1
  }
}
