use crate::problems::parameters::Parameters;
use crate::utils::fibonacci::Fibonacci;

pub fn solve(parameters: Parameters) -> String {
    if let Parameters::P2 { s1, s2, max } = parameters {
        let fibonacci = Fibonacci::new(s1, s2);
        let even_terms = fibonacci
            .take_while(|term| term <= &max)
            .filter(|term| term % 2 == 0)
            .collect::<Vec<u64>>();
        let sum = even_terms.iter().sum::<u64>();
        format!("{}", sum)
    } else {
        panic!("Invalid parameters")
    }
}
