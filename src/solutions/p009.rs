use super::Solve;

pub struct Parameters {
    pub n: usize,
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let mut a = 1;
        let mut b = 2;
        let mut c = n - a - b;

        let result = loop {
            if a.pow(2) + b.pow(2) == c.pow(2) {
                break Some((a, b, c));
            }
            if b + 1 >= c {
                if a + 1 >= b {
                    break None;
                }
                a += 1;
                b = a + 1;
            } else {
                b += 1;
            }
            c = n - a - b;
        };

        Ok(match result {
            Some((a, b, c)) => Some(format!("{}", a * b * c)),
            None => None,
        })
    }
}
