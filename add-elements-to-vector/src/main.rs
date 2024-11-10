fn main() {
    // Base vector
    let mut numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}", numbers);

    // 1. Extend with array slice
    numbers.extend(&[4, 5, 6]);
    println!("After extending with array slice: {:?}", numbers);

    // 2. Extend with another vector
    numbers.extend(vec![7, 8, 9]);
    println!("After extending with vector: {:?}", numbers);

    // 3. Extend with range
    numbers.extend(10..=12);
    println!("After extending with range: {:?}", numbers);

    // 4. Extend with iterator map
    numbers.extend((13..=15).map(|x| x * 2));
    println!("After extending with mapped range: {:?}", numbers);

    // 5. Extend with filter
    numbers.extend((16..=20).filter(|x| x % 2 == 0));
    println!("After extending with filtered range: {:?}", numbers);

    // 6. Extend with chain of iterators
    numbers.extend([21, 22].iter().chain([23, 24].iter()));
    println!("After extending with chained iterators: {:?}", numbers);

    // 7. Extend with collect from string
    let mut chars = Vec::new();
    chars.extend("Hello".chars());
    println!("Vector of chars: {:?}", chars);

    // 8. Extend with zip iterator
    let mut pairs = Vec::new();
    pairs.extend((1..=3).zip(vec!['a', 'b', 'c']));
    println!("Vector of pairs: {:?}", pairs);

    // 9. Extend with repeat
    let mut repeated = vec![1];
    repeated.extend(std::iter::repeat(0).take(3));
    println!("After extending with repeat: {:?}", repeated);

    // 10. Extend with once
    numbers.extend(std::iter::once(100));
    println!("After extending with once: {:?}", numbers);

    // Extend with Option values
    let mut opt_vec = Vec::new();
    opt_vec.extend([Some(1), None, Some(3)]);
    println!("Vector with Options: {:?}", opt_vec);

    // Extend with iterator of references
    let strings = vec!["hello", "world"];
    let mut string_vec: Vec<&str> = Vec::new();
    string_vec.extend(strings.iter());
    println!("Vector of string references: {:?}", string_vec);

    // Extend with collect from split
    let mut words = Vec::new();
    words.extend("hello world".split_whitespace());
    println!("Vector of words: {:?}", words);

    // Extend with enumerate
    let mut indexed = Vec::new();
    indexed.extend("abc".chars().enumerate());
    println!("Indexed characters: {:?}", indexed);

    add_both_ends(&mut numbers, 999);
    println!("Adding to both ends: {:?}", numbers);

    // vec2 doesn't need to be mutable
    let vec2 = vec![77, 88, 99];

    // Ownership of vec2 is moved to the function,
    // Once ownership is transferred, the function can modify the value
    // regardless of whether it was originally declared as mutable.
    append_vector(&mut numbers, vec2);
    println!("After appending second vector: {:?}", numbers);

}

// Function to add value at both ends of vector
fn add_both_ends<T>(vec: &mut Vec<T>, value: T)
    where T: Clone {
        vec.insert(0, value.clone()); // Add at beginning
        vec.push(value);             // Add at end
}

// Function to append one vector to another
// Added Debug trait bound for T
fn append_vector<T: std::fmt::Debug>(vec1: &mut Vec<T>, mut vec2: Vec<T>) {
    println!("vec2 before append: {:?}", vec2);
    vec1.append(&mut vec2);
    println!("vec2 after append: {:?}", vec2);
}

// Function to append one vector to another
// fn append_vector<T>(vec1: &mut Vec<T>, mut vec2: Vec<T>) {
//     vec1.append(&mut vec2);
//     println!("vec2: {:?}",vec2);
// }
