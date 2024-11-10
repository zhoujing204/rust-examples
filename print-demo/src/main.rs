#[derive(Debug, Clone)]
struct Person {
    first_name: String,
    last_name: String,
    age: u32
}

fn main() {
    let mut person1 = Person {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30
    };

    // Clone person1
    let mut person2 = person1.clone();

    // Modify person2
    person2.first_name = String::from("Jane");
    person2.age = 25;

    // person1 remains unchanged
    println!("Person 1: {:?}", person1);  // John Doe, 30
    println!("Person 2: {:?}", person2);  // Jane Doe, 25
}