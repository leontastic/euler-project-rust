use crate::problems::Problem;

mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;

use p1::PARAMETERS as P1;
use p2::PARAMETERS as P2;
use p3::PARAMETERS as P3;
use p4::PARAMETERS as P4;
use p5::PARAMETERS as P5;
use p6::PARAMETERS as P6;

pub enum Parameters {
    P1 { max: u64, a: u64, b: u64 },
    P2 { s1: u64, s2: u64, max: u64 },
    P3 { n: u64 },
    P4 { digits: u32 },
    P5 { n: u64 },
    P6 { n: u64 },
}

pub fn get_parameters(problem: &Problem) -> Parameters {
    match problem {
        Problem::P1 => P1,
        Problem::P2 => P2,
        Problem::P3 => P3,
        Problem::P4 => P4,
        Problem::P5 => P5,
        Problem::P6 => P6,
    }
}
