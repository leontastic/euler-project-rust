use crate::problems::parameters::Parameters;

pub fn solve(parameters: Parameters) -> String {
    if let Parameters::P6 { n } = parameters {
        let sum_of_squares = (1..n + 1).map(|x| x.pow(2)).sum::<u64>();
        let square_of_sum = (1..n + 1).sum::<u64>().pow(2);
        let result = square_of_sum - sum_of_squares;

        format!("{}", result)
    } else {
        panic!("Invalid parameters")
    }
}
