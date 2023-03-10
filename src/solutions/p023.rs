use std::collections::HashSet;
use std::iter::FromIterator;

use crate::utils::factors::factorize_proper;

use super::Solve;

pub struct Parameters {
    pub n: usize,
}

fn sum_of_proper_divisors(n: usize) -> usize {
    factorize_proper(n).iter().sum()
}

fn is_abundant(n: usize) -> bool {
    n < sum_of_proper_divisors(n)
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let abundant_numbers = (1..=n).filter(|&x| is_abundant(x)).collect::<Vec<usize>>();
        let abundant_sums = abundant_numbers
            .iter()
            .flat_map(|&a| abundant_numbers.iter().map(move |&b| a + b))
            .filter(|&n| n <= 28123)
            .collect::<HashSet<usize>>();
        let all_possible_sums = HashSet::<usize>::from_iter(1..=n);
        let non_abundant_sums = &all_possible_sums - &abundant_sums;

        let sum_of_non_abundant_sums = non_abundant_sums.iter().sum::<usize>();

        Ok(Some(format!("{}", sum_of_non_abundant_sums)))
    }
}
