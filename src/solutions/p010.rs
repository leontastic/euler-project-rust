use super::Solve;
use crate::utils::primes::Primes;

pub struct Parameters {
    pub n: u64,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let sum: u64 = 2 + Primes::new().take_while(|&prime| prime < n).sum::<u64>();

        Ok(Some(format!("{}", sum)))
    }
}
