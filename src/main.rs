mod parameters;
mod problems;
mod solutions;
mod utils;

use problems::*;
use solutions::Solve;

use std::env;
use std::process;
use std::str::FromStr;

const INTRO: &str = "
===============================================
  Project Euler Solutions in Rust, by Leon Li
===============================================
";

const USAGE_INSTRUCTIONS: &str = "Usage:\n\n    cargo run <problem number>\n";

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
        println!("Could not find problem {}\n", problem_number);
    }
}

fn main() {
    println!("{}", INTRO);

    let problem_number = match env::args().nth(1) {
        Some(arg) => usize::from_str(&arg).expect("Error parsing problem number"),
        None => {
            eprintln!("{}", USAGE_INSTRUCTIONS);
            process::exit(1)
        }
    };

    print_problem(problem_number);
}
