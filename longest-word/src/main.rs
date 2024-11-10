fn find_longest_word(sentence: &str) -> &str {
    sentence
        .split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    let sentence = String::from("The quickest-smartest brown fox jumping.");

    // Find longest word
    let longest = find_longest_word(&sentence);
    println!("Longest word: {}", longest);
}