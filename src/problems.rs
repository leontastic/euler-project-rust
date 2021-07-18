pub struct Problem {
    question: &'static str,
}

impl Problem {
    pub fn question(&self) -> &str {
        &self.question
    }
}

const P1: Problem = Problem {
    question: include_str!("problems/p1.txt"),
};
const P2: Problem = Problem {
    question: include_str!("problems/p2.txt"),
};
const P3: Problem = Problem {
    question: include_str!("problems/p3.txt"),
};

pub fn get_problem(number: &u16) -> Problem {
    match number {
        1 => P1,
        2 => P2,
        3 => P3,
        _ => panic!("Cannot find problem number {}", number),
    }
}
