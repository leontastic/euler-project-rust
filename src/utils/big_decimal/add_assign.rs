use super::BigDecimal;
use std::ops::AddAssign;

impl AddAssign<BigDecimal> for BigDecimal {
  fn add_assign(&mut self, other: Self) {
    *self = self.clone() + other;
  }
}

#[test]
fn add_assign_big_decimal_correctness() {
  let mut x = BigDecimal::from_str("1");
  x += BigDecimal::from_str("1");
  assert_eq!(x, BigDecimal::from_str("2"));
  x += BigDecimal::from_str("2");
  assert_eq!(x, BigDecimal::from_str("4"));
  x += BigDecimal::from_str("3");
  assert_eq!(x, BigDecimal::from_str("7"));

  let mut y = BigDecimal::from_str("89898989898989898989");
  y += BigDecimal::from_str("676767676767676767");
  assert_eq!(y, BigDecimal::from_str("90575757575757575756"))
}
