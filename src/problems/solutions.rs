use super::parameters::Parameters;
use super::Problem;

pub mod p1;
pub mod p2;
pub mod p3;

pub fn get_solution(problem: &Problem) -> fn(parameters: Parameters) -> String {
    match problem {
        Problem::P1 => p1::solve,
        Problem::P2 => p2::solve,
        Problem::P3 => p3::solve,
    }
}
