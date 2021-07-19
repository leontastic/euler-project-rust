use crate::utils::math::lcm;

fn summation(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn sum_multiples(n: u64, max: u64) -> u64 {
    let num_multiples = (max - 1) / n;
    n * summation(num_multiples)
}

pub fn solve() -> String {
    let max: u64 = 10;
    let a: u64 = 3;
    let b: u64 = 5;

    let sum_multiples_of_lcm = sum_multiples(lcm(a, b), max);
    let sum_multiples_of_a = sum_multiples(a, max);
    let sum_multiples_of_b = sum_multiples(b, max);
    let result = sum_multiples_of_a + sum_multiples_of_b - sum_multiples_of_lcm;

    format!("{}", result)
}
