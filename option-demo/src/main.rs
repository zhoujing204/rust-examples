fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);  // Some(5.0)
    let result2 = divide(10.0, 0.0);  // None

    // Safe ways to handle Option
    match result2 {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero")
    }

    // Using if let for simpler matching
    if let Some(x) = result1 {
        println!("Result: {}", x);
    }

    // Common methods
    let value1 = result1.unwrap();
    println!("value1: {}", value1);

    let value = result2.unwrap_or(0.0);         // Provides default if None
    let safe_value = result1.expect("Error message");  // Panics with message if None
    let mapped = result1.map(|x| x * 2.0);      // Transforms the contained value
}