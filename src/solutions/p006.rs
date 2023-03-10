use super::Solve;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;
        let sum_of_squares = (1..n + 1).map(|x| x.pow(2)).sum::<usize>();
        let square_of_sum = (1..n + 1).sum::<usize>().pow(2);
        let result = square_of_sum - sum_of_squares;

        Ok(Some(format!("{}", result)))
    }
}
