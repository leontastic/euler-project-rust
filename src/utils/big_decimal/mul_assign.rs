use super::BigDecimal;
use std::ops::MulAssign;

impl MulAssign<BigDecimal> for BigDecimal {
  fn mul_assign(&mut self, other: Self) {
    *self = self.clone() * other;
  }
}
