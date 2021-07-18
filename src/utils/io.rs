use std::io;
use std::io::Write;

pub fn read_u16() -> u16 {
    let mut buffer = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input");
    println!("");
    buffer
        .trim()
        .parse::<u16>()
        .expect("Could not parse number")
}
