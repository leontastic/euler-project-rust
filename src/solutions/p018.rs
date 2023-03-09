use super::Solve;
use std::cmp;

pub struct Parameters {
    pub input: &'static str,
}

fn normalize_input(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n")
        .filter(|&line| line.len() > 0)
        .map(|line| {
            line.split(" ")
                .map(|word| word.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>()
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { input } = *self;

        let rows = normalize_input(input);

        let maximum_row = rows.iter().rfold(vec![], |previous_maximums, row| {
            let current_maximums: Vec<usize> = if previous_maximums.len() == 0 {
                row.iter().map(|x| x.clone()).collect()
            } else {
                previous_maximums
                    .iter()
                    .zip(row.iter())
                    .map(|(&a, &b)| a + b)
                    .collect()
            };

            if current_maximums.len() == 1 {
                return current_maximums;
            }

            current_maximums
                .iter()
                .skip(1)
                .zip(current_maximums.iter())
                .map(|(&a, &b)| cmp::max(a, b))
                .collect()
        });

        Ok(Some(format!("{:?}", maximum_row[0])))
    }
}
