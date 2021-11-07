use super::BigDecimal;
use std::ops::Add;

impl Add for BigDecimal {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    let mut place = 0;
    let mut carry = 0;
    let mut sum_digits = Vec::<u8>::new();

    loop {
      if carry == 0 && place >= self.len() && place >= rhs.len() {
        break;
      }

      let mut sum_digit = self.get_digit(place) + rhs.get_digit(place) + carry;

      carry = 0;

      while sum_digit >= 10 {
        sum_digit -= 10;
        carry += 1;
      }

      sum_digits.push(sum_digit);

      place += 1;
    }

    Self(sum_digits)
  }
}

#[test]
fn add_big_decimal_correctness() {
  assert_eq!(
    BigDecimal::from_str("1") + BigDecimal::from_str("1"),
    BigDecimal::from_str("2")
  );
  assert_eq!(
    BigDecimal::from_str("2") + BigDecimal::from_str("2"),
    BigDecimal::from_str("4")
  );
  assert_eq!(
    BigDecimal::from_str("3") + BigDecimal::from_str("4"),
    BigDecimal::from_str("7")
  );
  assert_eq!(
    BigDecimal::from_str("8") + BigDecimal::from_str("9"),
    BigDecimal::from_str("17")
  );
  assert_eq!(
    BigDecimal::from_str("89898989898989898989") + BigDecimal::from_str("676767676767676767"),
    BigDecimal::from_str("90575757575757575756")
  )
}
