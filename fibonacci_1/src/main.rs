use std::io::{self, Write};
use std::time::Instant;
fn main() {
    print!("Please enter the fibonacci term you want: ");
    let term = loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input.parse::<u32>() {
            Ok(n) => break n,
            Err(_) => {
                print!("Please enter a valid number: ");
                continue;
            }
        }
    };
    let start_time = Instant::now();
    let fibonacci_number: u128 = fibonacci(term);
    let duration = start_time.elapsed();
    println!("The {}th term of the Fibonacci sequence is: {}. It took {} seconds", term, fibonacci_number, duration.as_secs_f64());
    print!("Press Enter to exit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn fibonacci(term: u32) -> u128 {
    if term == 0 {
        return 0;
    }
    if term == 1 {
        return 1;
    }
    fibonacci(term - 1) + fibonacci(term - 2)
}