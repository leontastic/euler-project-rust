use super::Solve;
use crate::utils::primes::Primes;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let sum: usize = 2 + Primes::new().take_while(|&prime| prime < n).sum::<usize>();

        Ok(Some(format!("{}", sum)))
    }
}
