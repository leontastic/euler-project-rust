use std::cmp::PartialEq;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigDecimal(pub Vec<u8>);

pub mod add;
pub mod add_assign;
pub mod display;
pub mod mul;
pub mod mul_assign;
pub mod sum;

// store decimal digits as vector of u8 ordered from least significant to most significant
impl BigDecimal {
    pub fn from_str(input: &str) -> BigDecimal {
        BigDecimal(
            input
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .rev()
                .collect(),
        )
    }

    pub fn get_digit(&self, n: usize) -> u8 {
        match self.0.get(n) {
            None => 0,
            Some(digit) => *digit,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[test]
fn partial_eq_big_decimal_correctness() {
    // implementations must ensure that eq and ne are consistent with each other
    assert_eq!(
        BigDecimal::from_str("12345678900987654321")
            .eq(&BigDecimal::from_str("12345678900987654321")),
        true
    );
    assert_eq!(
        BigDecimal::from_str("12345678900987654321")
            .eq(&BigDecimal::from_str("123456789009876543210987654321")),
        false
    );
    assert_eq!(
        BigDecimal::from_str("12345678900987654321")
            .ne(&BigDecimal::from_str("12345678900987654321")),
        false
    );
    assert_eq!(
        BigDecimal::from_str("12345678900987654321")
            .ne(&BigDecimal::from_str("123456789009876543210987654321")),
        true
    );
    assert_eq!(
        BigDecimal::from_str("12345678900987654321")
            .eq(&BigDecimal::from_str("12345678900987654321")),
        BigDecimal::from_str("12345678900987654321")
            == BigDecimal::from_str("12345678900987654321")
    );
    assert_eq!(
        BigDecimal::from_str("12345678900987654321")
            .ne(&BigDecimal::from_str("12345678900987654321")),
        BigDecimal::from_str("12345678900987654321")
            != BigDecimal::from_str("12345678900987654321")
    );
    assert_eq!(
        BigDecimal::from_str("12345678900987654321")
            .eq(&BigDecimal::from_str("123456789009876543210987654321")),
        BigDecimal::from_str("12345678900987654321")
            == BigDecimal::from_str("123456789009876543210987654321")
    );
    assert_eq!(
        BigDecimal::from_str("12345678900987654321")
            .ne(&BigDecimal::from_str("123456789009876543210987654321")),
        BigDecimal::from_str("12345678900987654321")
            != BigDecimal::from_str("123456789009876543210987654321")
    );
}

#[test]
fn eq_big_decimal_correctness() {
    assert_eq!(
        BigDecimal::from_str("12345678900987654321"),
        BigDecimal::from_str("12345678900987654321")
    );
}
