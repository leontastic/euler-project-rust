use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug)]
pub struct BigDecimal(pub Vec<u8>);

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

impl Display for BigDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            self.0
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

impl PartialEq for BigDecimal {
    fn eq(&self, rhs: &Self) -> bool {
        if self.len() != rhs.len() {
            return false;
        }

        let mut place = 0;

        loop {
            if place >= self.len() {
                break true;
            }

            if self.get_digit(place) != rhs.get_digit(place) {
                break false;
            }

            place += 1;
        }
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

impl Eq for BigDecimal {}

#[test]
fn eq_big_decimal_correctness() {
    assert_eq!(
        BigDecimal::from_str("12345678900987654321"),
        BigDecimal::from_str("12345678900987654321")
    );
}

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
