use std::io::{self, Write};
fn main() {
    let choice = option_fahrenheit_true();
    let value = get_value();
    if choice {
        println!("{} Fahrenheit is {} Celsius.", value, fahrenheit_to_celsius(value));
    } else {
        println!("{} Celsius is {} Fahrenheit.", value, celsius_to_fahrenheit(value));
    }
    print!("Press Enter to exit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn option_fahrenheit_true() -> bool {
    print!("Would you like to enter a temperature in Fahrenheit or Celsius? (F/C): ");
    loop {
        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        choice = choice.trim().to_uppercase();
        if choice.eq("F") || choice.eq("C") {
            return choice.eq("F")
        }
        print!("Invalid input. Read '{}'. Please enter 'F' for Fahrenheit or 'C' for Celsius: ", choice);
    }
}

fn get_value() -> f64 {
    print!("Please enter the temperature value: ");
    loop {
        let mut value = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).unwrap();
        match value.trim().parse::<f64>() {
            Ok(v) => return v,
            Err(_) => {
                print!("Invalid input. Read '{}'. Please enter a valid value: ", value.trim());
                continue;
            }
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}