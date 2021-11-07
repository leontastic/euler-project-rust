use super::BigDecimal;
use std::iter::Sum;

impl Sum<BigDecimal> for BigDecimal {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = BigDecimal>,
  {
    iter.fold(BigDecimal(vec![0]), |sum, num| sum + num)
  }
}

#[test]
fn sum_big_decimal_correctness() {
  assert_eq!(
    ["1", "2", "3"]
      .iter()
      .map(|x| BigDecimal::from_str(&x))
      .sum::<BigDecimal>(),
    BigDecimal::from_str("6")
  );
  assert_eq!(
    [
      "89898989898989898989",
      "676767676767676767",
      "90575757575757575756"
    ]
    .iter()
    .map(|x| BigDecimal::from_str(&x))
    .sum::<BigDecimal>(),
    BigDecimal::from_str("181151515151515151512")
  );
}
