use super::Solve;
use crate::utils::math::lcm;

pub struct Parameters {
    pub max: u64,
    pub a: u64,
    pub b: u64,
}

fn summation(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn sum_multiples(n: u64, max: u64) -> u64 {
    let num_multiples = (max - 1) / n;
    n * summation(num_multiples)
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { max, a, b } = *self;

        let sum_multiples_of_lcm = sum_multiples(lcm(a, b), max);
        let sum_multiples_of_a = sum_multiples(a, max);
        let sum_multiples_of_b = sum_multiples(b, max);
        let result = sum_multiples_of_a + sum_multiples_of_b - sum_multiples_of_lcm;

        Ok(Some(format!("{}", result)))
    }
}
