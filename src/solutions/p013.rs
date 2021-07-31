use super::Solve;
use crate::utils::big_decimal::BigDecimal;

pub struct Parameters {
    pub input: [&'static str; 100],
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { input } = *self;

        let BigDecimal(digits) = input
            .iter()
            .map(|s| BigDecimal::from_str(s))
            .sum::<BigDecimal>();

        let first_ten: String = digits
            .iter()
            .rev()
            .take(10)
            .map(|&digit| char::from_digit(digit as u32, 10).unwrap())
            .collect();

        Ok(Some(format!("{}", first_ten)))
    }
}
