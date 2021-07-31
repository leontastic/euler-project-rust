use super::parameters::*;
use super::solutions::Solve;

pub const PROBLEMS: [Problem; 12] = [
    Problem(include_str!("questions/p001.txt"), &p001::PARAMETERS),
    Problem(include_str!("questions/p002.txt"), &p002::PARAMETERS),
    Problem(include_str!("questions/p003.txt"), &p003::PARAMETERS),
    Problem(include_str!("questions/p004.txt"), &p004::PARAMETERS),
    Problem(include_str!("questions/p005.txt"), &p005::PARAMETERS),
    Problem(include_str!("questions/p006.txt"), &p006::PARAMETERS),
    Problem(include_str!("questions/p007.txt"), &p007::PARAMETERS),
    Problem(include_str!("questions/p008.txt"), &p008::PARAMETERS),
    Problem(include_str!("questions/p009.txt"), &p009::PARAMETERS),
    Problem(include_str!("questions/p010.txt"), &p010::PARAMETERS),
    Problem(include_str!("questions/p011.txt"), &p011::PARAMETERS),
    Problem(include_str!("questions/p012.txt"), &p012::PARAMETERS),
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
