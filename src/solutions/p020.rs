use crate::utils::big_decimal::BigDecimal;

use super::Solve;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let factorial = (1..=n)
            .map(|x| BigDecimal::from_usize(x))
            .reduce(|a, b| a * b)
            .unwrap();

        let BigDecimal(digits) = factorial;

        let sum_digits = digits.into_iter().map(|x| x as usize).sum::<usize>();

        Ok(Some(format!("{}", sum_digits)))
    }
}
