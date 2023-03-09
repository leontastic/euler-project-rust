use super::Solve;
use crate::utils::big_decimal::BigDecimal;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let mut decimal = BigDecimal::from_usize(2);

        for _ in 1..n {
            decimal *= BigDecimal::from_usize(2);
        }

        let BigDecimal(digits) = decimal;

        let sum = digits.iter().map(|&digit| digit as usize).sum::<usize>();

        Ok(Some(format!("{}", sum)))
    }
}
