pub struct TriangleNumbers(usize, usize);

impl TriangleNumbers {
    pub fn new() -> TriangleNumbers {
        TriangleNumbers(0, 0)
    }
}

impl Iterator for TriangleNumbers {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.1 += 1;
        self.0 += self.1;
        Some(self.0)
    }
}
