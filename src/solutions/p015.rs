use super::Solve;
use crate::utils::pascal::Pascal;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let mut pascal = Pascal::new();
        let pascal_row = &pascal.nth(n * 2).unwrap();
        let diamond_number = pascal_row.into_iter().nth(pascal_row.len() / 2).unwrap();

        Ok(Some(format!("{}", diamond_number)))
    }
}
