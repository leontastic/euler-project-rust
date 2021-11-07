use super::BigDecimal;
use std::fmt::{Display, Formatter, Result};

impl Display for BigDecimal {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "{}",
      self
        .0
        .iter()
        .map(|x| x.to_string())
        .rev()
        .collect::<Vec<String>>()
        .join("")
    )
  }
}

#[test]
fn display_big_decimal_correctness() {
  assert_eq!(format!("{}", BigDecimal::from_str("0")), "0");
  assert_eq!(format!("{}", BigDecimal::from_str("1")), "1");
  assert_eq!(format!("{}", BigDecimal::from_str("99")), "99");
  assert_eq!(
    format!("{}", BigDecimal::from_str("12345678900987654321")),
    "12345678900987654321"
  );
}
