mod parameters;
mod questions;
mod solutions;

use parameters::Parameters;
use std::fmt;

pub enum Problem {
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
}

impl Problem {
    fn question(&self) -> &str {
        questions::get_question(self)
    }
    fn parameters(&self) -> Parameters {
        parameters::get_parameters(self)
    }

    pub fn solve(&self) -> String {
        let solution = solutions::get_solution(self);
        solution(self.parameters())
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.question())
    }
}

pub fn get_problem(number: &u16) -> Problem {
    match number {
        1 => Problem::P1,
        2 => Problem::P2,
        3 => Problem::P3,
        4 => Problem::P4,
        5 => Problem::P5,
        6 => Problem::P6,
        7 => Problem::P7,
        8 => Problem::P8,
        _ => panic!("Cannot find problem number {}", number),
    }
}
