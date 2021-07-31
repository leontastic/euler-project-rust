pub struct Collatz(usize, bool);

impl Collatz {
    pub fn new(n: usize) -> Self {
        Self(n, false)
    }
}

impl Iterator for Collatz {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == false {
            self.1 = true;
            return Some(self.0);
        }

        if self.0 == 1 {
            None
        } else {
            if self.0 % 2 == 0 {
                self.0 /= 2;
            } else {
                self.0 = self.0 * 3 + 1;
            }
            Some(self.0)
        }
    }
}

#[test]
fn collatz_correctness() {
    let collatz: Vec<usize> = Collatz::new(13).collect();
    assert_eq!(collatz.len(), 10);
    assert_eq!(collatz, vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
}
