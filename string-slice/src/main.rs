fn main() {
    let mut owned_string = String::from("hello");
    let mut owned_string2 = String::from("other thing");

    let mut slice = &owned_string;  // immutable borrow
    // owned_string.push_str(" world");  // This would cause a compile error!
    // Cannot modify owned_string while slice is borrowed
    println!("{}", slice);  // Use of slice
    let mut slice2 = slice;

    // You CAN do this (change what the slice refers to):
    slice = &owned_string2;
    slice2 = &String::from("another string");
    println!("{}", slice);  // Use of slice
    // slice's lifetime ends here - after its last usage

    // You CANNOT do this (modify the content):
    // slice.push_str(" world");     // ERROR: &str doesn't have push_str method
    // *slice = "new content";

    owned_string.push_str(" world");  // OK after slice is no longer used
    println!("{}",owned_string);

    println!("{}", slice);  // Use of slice


}
