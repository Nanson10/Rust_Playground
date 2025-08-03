fn main() {
    let string = String::from("Hello, world!");
    let string_literal = "Hello, world!";
    println!("String: '{}'", first_word(&string[..]));
    println!("String literal: '{}'", first_word(string_literal));
}

fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[..i];
        }
    }
    string
}