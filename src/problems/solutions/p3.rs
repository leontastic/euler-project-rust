use crate::problems::parameters::Parameters;
use crate::utils::primes::Primes;
use std::cmp;

pub fn solve(parameters: Parameters) -> String {
    if let Parameters::P3 { mut n } = parameters {
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
        if max_prime > 0 {
            format!("{}", cmp::max(max_prime, n))
        } else {
            panic!("No prime factors found")
        }
    } else {
        panic!("Invalid parameters")
    }
}
