use super::Solve;
use crate::utils::primes::Primes;
use std::cmp;

pub struct Parameters {
    pub n: u64,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { mut n } = *self;

        let primes = Primes::new();
        let mut max_prime: u64 = 0;
        for prime in primes {
            if prime * prime > n {
                break;
            }
            if n % prime == 0 {
                while n % prime == 0 {
                    n /= prime;
                }
                max_prime = prime;
            }
        }

        Ok(if max_prime > 0 {
            Some(format!("{}", cmp::max(max_prime, n)))
        } else {
            None
        })
    }
}
