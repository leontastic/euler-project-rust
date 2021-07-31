use super::Solve;
use crate::utils::collatz::Collatz;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let collatz_lengths = (1..n)
            .map(|n| Collatz::new(n).count())
            .collect::<Vec<usize>>();

        let (longest_collatz_number, _) = (1..n)
            .zip(collatz_lengths)
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        Ok(Some(format!("{}", longest_collatz_number)))
    }
}
