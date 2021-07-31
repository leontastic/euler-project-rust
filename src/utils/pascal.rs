pub struct Pascal(Vec<u64>);

impl Pascal {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Iterator for Pascal {
    type Item = Vec<u64>;
    fn next(&mut self) -> Option<Vec<u64>> {
        if self.0.len() <= 1 {
            self.0.push(1);
        } else {
            let new_row = (1..self.0.len())
                .map(|i| self.0[i - 1] + self.0[i])
                .collect::<Vec<u64>>();
            let new_row = [vec![1], new_row, vec![1]].concat();
            self.0 = new_row;
        }

        Some(self.0.clone())
    }
}

#[test]
fn pascal_correctness() {
    let pascal = Pascal::new();
    let cases: [Vec<u64>; 10] = [
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
        vec![1, 6, 15, 20, 15, 6, 1],
        vec![1, 7, 21, 35, 35, 21, 7, 1],
        vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
        vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
    ];

    for (expected, actual) in cases.iter().zip(pascal) {
        assert_eq!(*expected, actual);
    }
}
