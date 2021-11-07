use super::BigDecimal;
use std::ops::Mul;

impl Mul<BigDecimal> for BigDecimal {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    self
      .0
      .iter()
      .map(|v| other.0.iter().map(move |w| v * w))
      .enumerate()
      .map(|(i, v_i)| {
        v_i
          .enumerate()
          .map(|(j, vw)| Self::from_str(&(vw.to_string() + &"0".repeat(i + j))))
          .sum::<BigDecimal>()
      })
      .sum::<BigDecimal>()
  }
}

#[test]
fn mul_big_decimal_correctness() {
  assert_eq!(
    BigDecimal::from_str("2") * BigDecimal::from_str("2"),
    BigDecimal::from_str("4")
  );
  assert_eq!(
        BigDecimal::from_str("46488416629377247632918291158238061014866615871675152913624137303215071933136")
        * BigDecimal::from_str("10183905701066272406597175934640315096969880045504139889331864776427950304382965949295737413038440063537216904716682"),
        BigDecimal::from_str("473433651145459055497486344726446507336504084196668109294981229997455186743014814867177030194263939656882480956773961004110243846170516927332198657059883076562417486973363788367401114127774752")
    );
}
