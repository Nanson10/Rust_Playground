use std::io::{self, Write};
use std::cmp::Ordering;
use std::num::ParseIntError;
use rand::Rng;
fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    let mut tries: u32 = 0;
    println!("Try and guess the number!");
    println!("The secret number is between 1 and 100. Good Luck!");
    print!("Please input your guess: ");
    loop {
        let mut guess = String::new();
        io::stdout().flush().expect("Failed to flush stdout (trying to print before prompting for input)");
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {},
            Err(_) => {
                print!("Failed to read line. Please try again: ");
                continue;
            }
        }
        let guess: Result<i32, ParseIntError> = guess.trim().parse();
        match guess {
            Ok(_) => {},
            Err(_) => {
                print!("Please enter a valid number: ");
                continue;
            }
        }
        let guess: i32 = guess.unwrap();
        if guess < 1 || guess > 100 {
            print!("Your guess is out of range. Please enter a number between 1 and 100: ");
            continue;
        }
        print!("You guessed {} which is ", guess);
        tries += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("too small. Try again: "),
            Ordering::Greater => print!("too big. Try again: "),
            Ordering::Equal => {
                print!("correct! The secret number was {}. You guessed the number in {} ", secret_number, tries);
                if tries == 1 {
                    println!("try!");
                } else {
                    println!("tries.");
                }
                break;
            }
        }
    }
}