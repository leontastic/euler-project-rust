use crate::problems::parameters::Parameters;

pub fn solve(parameters: Parameters) -> String {
    if let Parameters::PX { a, b, c } = parameters {
        let sum = a + b + c;
        format!("{}", sum)
    } else {
        panic!("Invalid parameters")
    }
}
