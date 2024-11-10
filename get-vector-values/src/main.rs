fn sum_vector(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

// Alternative implementation
fn sum_vector_manual(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for &num in vec {
        sum += num;
    }
    sum
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let num_sum = sum_vector(&numbers);
    println!("{}", num_sum);

    // Method 1: Direct indexing (can panic!)
    let third = numbers[2];  // Index 2 = third element

    // Method 2: Using .last() (returns Option)
    let last = numbers.last().unwrap();  // Safe with unwrap if vector not empty
    let first = numbers.first().unwrap();
    let third = numbers.get(2).unwrap();

    println!("last, first, third: {}, {}, {}", last, first, third);

    // Method 3: Using .get() (returns Option)
    match numbers.get(0) {
        Some(value) => println!("First value: {}", value),
        None => println!("Vector is empty"),
    }

    let empty_vec: Vec<i32> = vec![];

    // These will panic:
    //let value = empty_vec[0];         // Panic: index out of bounds
    //let last = empty_vec.last().unwrap();  // Panic: unwrap on None

    // Safe alternatives:
    match empty_vec.get(0) {
        Some(value) => println!("First value: {}", value),
        None => println!("Vector is empty"),
    }
}