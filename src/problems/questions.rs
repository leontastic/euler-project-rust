use crate::problems::Problem;

pub fn get_question(problem: &Problem) -> &str {
    match problem {
        Problem::P1 => include_str!("questions/p1.txt"),
        Problem::P2 => include_str!("questions/p2.txt"),
        Problem::P3 => include_str!("questions/p3.txt"),
    }
}