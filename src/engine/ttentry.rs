
  #[derive(Clone, Copy)]
  pub struct TtEntry {
      pub score: i32,
      pub depth: u8,
      pub flag:  TtFlag,
  }

  #[derive(Clone, Copy, PartialEq)]
  pub enum TtFlag { Exact, LowerBound, UpperBound }
