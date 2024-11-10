// Using slices for multiple values
fn process_numbers(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// Using vectors
fn process_with_vector(nums: Vec<i32>) -> i32 {
    nums.iter().sum()
}

fn main() {
    // With slice
    let numbers = [1, 2, 3, 4, 5];
    let sum1 = process_numbers(&numbers);
    println!("{}", sum1);

    // With vector
    let vec_numbers = vec![1, 2, 3, 4, 5];
    let sum2 = process_with_vector(vec_numbers);
    println!("{}", sum2);
}