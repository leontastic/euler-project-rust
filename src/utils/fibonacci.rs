pub struct Fibonacci(u64, u64);

impl Fibonacci {
    pub fn new(s1: u64, s2: u64) -> Fibonacci {
        Fibonacci(s1, s2)
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.0 + self.1;
        self.0 = self.1;
        self.1 = next;
        Some(self.0)
    }
}
