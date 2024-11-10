#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>  // Optional age field
}



fn main() {
    println!("Hello, world!");
    // Using None
    let person1 = Person {
        first_name: "Sanchez".to_string(),
        last_name: "Smith".to_string(),
        age: None   // Optional age field can't be missed.
    };

    // Using Some
    let person2 = Person {
        first_name: "Sanchez".to_string(),
        last_name: "Smith".to_string(),
        age: Some(23)
    };

    println!("Person 1: {:?}", person1);
    println!("Person 2: {:?}", person2);
}
