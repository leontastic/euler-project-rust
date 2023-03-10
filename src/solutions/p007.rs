use super::Solve;
use crate::utils::primes::Primes;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        Ok(if let Some(result) = Primes::new().nth(n as usize - 2) {
            Some(format!("{}", result))
        } else {
            None
        })
    }
}
