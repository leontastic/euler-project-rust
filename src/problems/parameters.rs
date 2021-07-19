use crate::problems::Problem;

pub enum Parameters {
    P1 { max: u64, a: u64, b: u64 },
    P2 { s1: u64, s2: u64, max: u64 },
    P3 { n: u64 },
}

pub fn get_parameters(problem: &Problem) -> Parameters {
    match problem {
        Problem::P1 => Parameters::P1 {
            max: 1000,
            a: 3,
            b: 5,
        },
        Problem::P2 => Parameters::P2 {
            s1: 1,
            s2: 2,
            max: 4000000,
        },
        Problem::P3 => Parameters::P3 { n: 600851475143 },
    }
}
