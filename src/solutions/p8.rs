use super::Solve;

pub struct Parameters {
    pub n: u64,
    pub input: &'static str,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n, input } = *self;

        let result = (0..input.len() - n as usize)
            .map(|i| {
                input[i..i + n as usize]
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .fold(1, |product, digit| product * digit)
            })
            .max()
            .unwrap();

        Ok(Some(format!("{}", result)))
    }
}
