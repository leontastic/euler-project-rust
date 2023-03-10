use std::ops::Add;

pub struct Fibonacci<T: Add<Output = T> + Clone>(T, T);

impl<T: Add<Output = T> + Clone> Fibonacci<T> {
    pub fn new(s1: T, s2: T) -> Fibonacci<T> {
        Fibonacci(s1, s2)
    }
}

impl<T: Add<Output = T> + Clone> Iterator for Fibonacci<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let next = self.0.clone() + self.1.clone();
        self.0 = self.1.clone();
        self.1 = next;
        Some(self.0.clone())
    }
}
