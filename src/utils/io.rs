use std::io;
use std::io::Write;

pub fn read_natural() -> usize {
    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Could not read input");
        println!("");
        if let Ok(parsed) = buffer.trim().parse::<usize>() {
            if parsed > 0 {
                break parsed;
            } else {
                println!("Invalid number\n");
            }
        } else {
            println!("Invalid number\n");
        }
    }
}
