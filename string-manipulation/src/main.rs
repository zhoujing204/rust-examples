fn main() {

    // String Slicing
    println!("--------------------------");
    let sentence = String::from("the quick brown fox");
    // Get first three characters
    let slice = &sentence[0..3];  // "the"
    // Inclusive range
    let slice = &sentence[0..=3]; // "the "
    println!("String Slicing[0..=3]: {}",slice);

    // String Concatenation using format! macro
    println!("--------------------------");
    let sentence = String::from("hello");
    let result = format!("My string: {}", sentence);
    println!("String Concatenation result: {}",result);

    //Iterating String
    println!("--------------------------");
    let sentence = String::from("hello world");
    println!("Iterating sentence:");
    for c in sentence.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            println!("got a vowel {}", c);
        }
    }

    //Splitting String into Words
    println!("--------------------------");
    let sentence = String::from("the quick brown fox");
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("Splitting String into Words: {:?}", words);

    let words = sentence.split(" ").collect::<Vec<&str>>();
    println!("Splitting String into Words2: {:?}", words);

    let words: Vec<String> = sentence
    .split_whitespace()
    .map(String::from)  // Convert each &str to String
    .collect();
    println!("Splitting String into Words 3: {:?}", words);

    //Reversing String
    println!("--------------------------");
    let sentence = String::from("hello");
    let reversed: String = sentence.chars().rev().collect();
    println!("Reversing String: {}", reversed);

}
