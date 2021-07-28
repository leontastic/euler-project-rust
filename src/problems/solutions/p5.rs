use crate::problems::parameters::Parameters;
use crate::utils::math::lcm;

pub fn solve(parameters: Parameters) -> String {
    if let Parameters::P5 { n } = parameters {
        let result = (1..n + 1).fold(1, lcm);
        format!("{}", result)
    } else {
        panic!("Invalid parameters")
    }
}
