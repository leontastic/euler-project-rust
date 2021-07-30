use super::parameters::*;
use super::solutions::Solve;

pub const PROBLEMS: [Problem; 9] = [
    Problem(include_str!("questions/p1.txt"), &p1::PARAMETERS),
    Problem(include_str!("questions/p2.txt"), &p2::PARAMETERS),
    Problem(include_str!("questions/p3.txt"), &p3::PARAMETERS),
    Problem(include_str!("questions/p4.txt"), &p4::PARAMETERS),
    Problem(include_str!("questions/p5.txt"), &p5::PARAMETERS),
    Problem(include_str!("questions/p6.txt"), &p6::PARAMETERS),
    Problem(include_str!("questions/p7.txt"), &p7::PARAMETERS),
    Problem(include_str!("questions/p8.txt"), &p8::PARAMETERS),
    Problem(include_str!("questions/p9.txt"), &p9::PARAMETERS),
];

pub struct Problem(&'static str, &'static dyn Solve);

impl Solve for Problem {
    fn solve(&self) -> Result<Option<String>, &str> {
        self.1.solve()
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
