use super::parameters::Parameters;
use super::Problem;

pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;

pub fn get_solution(problem: &Problem) -> fn(parameters: Parameters) -> String {
    match problem {
        Problem::P1 => p1::solve,
        Problem::P2 => p2::solve,
        Problem::P3 => p3::solve,
        Problem::P4 => p4::solve,
        Problem::P5 => p5::solve,
        Problem::P6 => p6::solve,
        Problem::P7 => p7::solve,
    }
}
