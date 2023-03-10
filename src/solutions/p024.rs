use super::Solve;

pub struct Parameters {
    pub n: usize,
}

fn permutations_recursive(canonical: Vec<usize>) -> Vec<Vec<usize>> {
    if canonical.len() == 1 {
        vec![canonical]
    } else {
        canonical
            .iter()
            .flat_map(|&i| {
                let unused = canonical
                    .iter()
                    .cloned()
                    .filter(|&element| element != i)
                    .collect::<Vec<usize>>();

                permutations_recursive(unused)
                    .iter()
                    .map(|permutation| {
                        let mut full_permutation = permutation.clone();
                        full_permutation.insert(0, i);
                        full_permutation
                    })
                    .collect::<Vec<Vec<usize>>>()
            })
            .collect::<Vec<Vec<usize>>>()
    }
}

#[test]
fn permutations_recursive_correctness() {
    assert_eq!(
        permutations_recursive(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]
    );
    assert_eq!(
        permutations_recursive(vec![1, 2, 3, 4]),
        vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![1, 4, 3, 2],
            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![2, 4, 3, 1],
            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],
            vec![4, 1, 2, 3],
            vec![4, 1, 3, 2],
            vec![4, 2, 1, 3],
            vec![4, 2, 3, 1],
            vec![4, 3, 1, 2],
            vec![4, 3, 2, 1],
        ]
    );
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { n } = *self;

        let result = &permutations_recursive(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
            .into_iter()
            .skip(n - 1)
            .take(1)
            .collect::<Vec<Vec<usize>>>()[0]
            .iter()
            .map(|digit| digit.to_string())
            .collect::<Vec<String>>()
            .join("");

        Ok(Some(format!("{}", result)))
    }
}
