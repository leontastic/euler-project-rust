mod parameters;
mod problems;
mod solutions;
mod utils;

use problems::*;
use solutions::Solve;
use utils::io;

const INTRO: &str = "
===============================================
  Project Euler Solutions in Rust, by Leon Li
===============================================
";

fn prompt_problem_number() -> usize {
    println!("> Enter the problem number you would like to see: ");
    io::read_natural()
}

fn solve(problem: &Problem) -> String {
    match problem.solve() {
        Ok(Some(solution)) => format!("{}", solution),
        Ok(None) => format!("NO SOLUTION"),
        Err(error) => String::from(error),
    }
}

fn print_problem(problem_number: usize) {
    if let Some(problem) = PROBLEMS.iter().nth(problem_number - 1) {
        println!("PROBLEM {}:\n", problem_number);
        println!("{}", problem);
        println!("SOLUTION FOR PROBLEM {}:\n", problem_number);
        println!("{}", solve(problem));
    } else {
        println!("Could not find problem {}", problem_number);
    }
}

fn main() {
    println!("{}", INTRO);
    loop {
        println!("");
        let problem_number = prompt_problem_number();
        print_problem(problem_number);
    }
}
