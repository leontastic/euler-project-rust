use super::Solve;

pub struct Parameters {
    pub n: u64,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;
        let sum_of_squares = (1..n + 1).map(|x| x.pow(2)).sum::<u64>();
        let square_of_sum = (1..n + 1).sum::<u64>().pow(2);
        let result = square_of_sum - sum_of_squares;

        Ok(Some(format!("{}", result)))
    }
}
