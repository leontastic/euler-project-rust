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
    pub fn from_usize(input: usize) -> BigDecimal {
        BigDecimal::from_be_bytes(
            input
                .to_be_bytes()
                .iter()
                .cloned()
                .skip_while(|&byte| byte == 0)
                .collect(),
        )
    }

    pub fn from_be_bytes(bytes: Vec<u8>) -> BigDecimal {
        let len = bytes.len();
        let mut result = BigDecimal::from_u8(0);
        for (index, byte) in bytes.iter().enumerate() {
            let mut from_byte = BigDecimal::from_u8(*byte);
            for _ in (index + 1)..len {
                from_byte *= BigDecimal::from_str("256")
            }
            result += from_byte
        }
        result
    }

    pub fn from_u8(input: u8) -> BigDecimal {
        if input < 10 {
            BigDecimal(vec![input])
        } else {
            BigDecimal::from_u8(input.div_euclid(10)) * BigDecimal::from_str("10")
                + BigDecimal(vec![input.rem_euclid(10)])
        }
    }

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
fn from_usize_correctness() {
    assert_eq!(
        BigDecimal::from_usize(usize::MAX),
        BigDecimal::from_str(&format!("{}", usize::MAX))
    )
}

#[test]
fn from_be_bytes_correctness() {
    assert_eq!(
        BigDecimal::from_be_bytes(65535u16.to_be_bytes().to_vec()),
        BigDecimal::from_str("65535")
    );
    assert_eq!(
        BigDecimal::from_be_bytes(usize::MAX.to_be_bytes().to_vec()),
        BigDecimal::from_str(&format!("{}", usize::MAX))
    )
}

#[test]
fn from_u8_correctness() {
    assert_eq!(BigDecimal::from_u8(255u8), BigDecimal::from_str("255"));
    assert_eq!(
        BigDecimal::from_u8(255u8) * BigDecimal::from_u8(255u8),
        BigDecimal::from_str("65025")
    );
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
