// src/lib.rs

use std::io::{self, Write};

pub fn get_user_input_safe(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Usage with error handling
pub fn example_usage() {
    match get_user_input_safe("Enter something: ") {
        Ok(input) => println!("You entered: {}", input),
        Err(e) => eprintln!("Error: {}", e),
    }
}
