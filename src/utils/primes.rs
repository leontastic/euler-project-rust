pub struct Primes(Vec<usize>);

impl Primes {
    pub fn new() -> Primes {
        let primes = Vec::<usize>::new();
        Primes(primes)
    }
}

impl Iterator for Primes {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let primes = &self.0;
        if let Some(last) = primes.last() {
            let mut n = last + 2;
            while has_factor(n, &primes) {
                n += 2;
            }
            self.0.push(n);
            Some(n)
        } else {
            self.0.push(3);
            Some(3)
        }
    }
}

fn has_factor(n: usize, candidates: &Vec<usize>) -> bool {
    candidates
        .into_iter()
        .take_while(|&candidate| n >= candidate * candidate)
        .any(|candidate| n % candidate == 0)
}
