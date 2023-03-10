use crate::utils::factors::factorize_proper;

use super::Solve;

pub struct Parameters {
    pub n: usize,
}

fn sum_of_proper_divisors(n: usize) -> usize {
    factorize_proper(n).iter().sum()
}

fn is_amicable(a: usize) -> bool {
    let b = sum_of_proper_divisors(a);
    a != b && a == sum_of_proper_divisors(b)
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let sum_amicable_numbers = (1..n).filter(|&x| is_amicable(x)).sum::<usize>();

        Ok(Some(format!("{}", sum_amicable_numbers)))
    }
}
