use crate::utils::{big_decimal::BigDecimal, fibonacci::Fibonacci};

use super::Solve;

pub struct Parameters {
    pub digits: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { digits } = *self;

        let result = &Fibonacci::new(BigDecimal::from_usize(1), BigDecimal::from_usize(1))
            .into_iter()
            .enumerate()
            .skip_while(|(_, f)| f.len() != digits)
            .take(1)
            .collect::<Vec<(usize, BigDecimal)>>()[0];

        Ok(Some(format!("{}", result.0 + 2)))
    }
}
