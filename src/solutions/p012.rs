use super::Solve;
use crate::utils::factors::factorize;
use crate::utils::triangle_numbers::TriangleNumbers;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let triangle_number = TriangleNumbers::new()
            .skip_while(|&triangle_number| factorize(triangle_number).len() <= n)
            .next()
            .unwrap();

        Ok(Some(format!("{}", triangle_number)))
    }
}
