use crate::problems::parameters::Parameters;
use crate::utils::primes::Primes;

pub fn solve(parameters: Parameters) -> String {
    if let Parameters::P7 { n } = parameters {
        if let Some(result) = Primes::new().nth(n as usize - 2) {
            format!("{}", result)
        } else {
            panic!("Cannot find nth prime: {}", n)
        }
    } else {
        panic!("Invalid parameters")
    }
}
