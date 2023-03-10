use super::Solve;
use crate::utils::math::lcm;

pub struct Parameters {
    pub max: usize,
    pub a: usize,
    pub b: usize,
}

fn summation(n: usize) -> usize {
    n * (n + 1) / 2
}

fn sum_multiples(n: usize, max: usize) -> usize {
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
