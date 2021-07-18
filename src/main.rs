mod problems;
mod utils;
use utils::io;

fn prompt_problem_number() -> u16 {
    println!("> Enter the problem number you would like to see: ");
    io::read_u16()
}

fn main() {
    println!(" ============================================= ");
    println!("| Project Euler Solutions in Rust, by Leon Li |");
    println!(" ============================================= ");
    println!("");
    loop {
        let problem_number = prompt_problem_number();
        let problem = problems::get_problem(&problem_number);
        println!("PROBLEM {}:\n", problem_number);
        println!("{}", problem.question());
    }
}
