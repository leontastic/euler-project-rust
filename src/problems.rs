mod questions;
mod solutions;

use std::fmt;

pub enum Problem {
    P1,
    P2,
    P3,
}

impl Problem {
    fn question(&self) -> &str {
        match self {
            Problem::P1 => questions::Q1,
            Problem::P2 => questions::Q2,
            Problem::P3 => questions::Q3,
        }
    }

    pub fn solve(&self) -> String {
        match self {
            Problem::P1 => solutions::p1::solve(),
            Problem::P2 => solutions::p2::solve(),
            Problem::P3 => solutions::p3::solve(),
        }
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
        _ => panic!("Cannot find problem number {}", number),
    }
}
