use super::Solve;

pub struct Parameters {
    pub s1: u64,
    pub s2: u64,
    pub max: u64,
}

use crate::utils::fibonacci::Fibonacci;

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { s1, s2, max } = *self;

        let fibonacci = Fibonacci::new(s1, s2);
        let even_terms = fibonacci
            .take_while(|term| term <= &max)
            .filter(|term| term % 2 == 0)
            .collect::<Vec<u64>>();
        let sum = even_terms.iter().sum::<u64>();

        Ok(Some(format!("{}", sum)))
    }
}
