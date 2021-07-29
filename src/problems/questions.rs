use crate::problems::Problem;

pub fn get_question(problem: &Problem) -> &str {
    match problem {
        Problem::P1 => include_str!("questions/p1.txt"),
        Problem::P2 => include_str!("questions/p2.txt"),
        Problem::P3 => include_str!("questions/p3.txt"),
        Problem::P4 => include_str!("questions/p4.txt"),
        Problem::P5 => include_str!("questions/p5.txt"),
        Problem::P6 => include_str!("questions/p6.txt"),
        Problem::P7 => include_str!("questions/p7.txt"),
        Problem::P8 => include_str!("questions/p8.txt"),
    }
}
