pub struct Fibonacci(usize, usize);

impl Fibonacci {
    pub fn new(s1: usize, s2: usize) -> Fibonacci {
        Fibonacci(s1, s2)
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let next = self.0 + self.1;
        self.0 = self.1;
        self.1 = next;
        Some(self.0)
    }
}
