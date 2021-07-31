pub struct TriangleNumbers(u64, u64);

impl TriangleNumbers {
    pub fn new() -> TriangleNumbers {
        TriangleNumbers(0, 0)
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.1 += 1;
        self.0 += self.1;
        Some(self.0)
    }
}
