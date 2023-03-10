use super::Solve;
use crate::utils::math::lcm;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let result = (1..n + 1).fold(1, lcm);

        Ok(Some(format!("{}", result)))
    }
}
