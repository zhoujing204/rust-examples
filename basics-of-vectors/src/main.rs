fn main() {

    // Vector: growable, owned collection
    let mut vec = vec![1, 2, 3];    // Vector

    // Slice: fixed-size view into a collection
    let slice = &vec[..];           // Slice of all elements

    // Vector (owned)
    let numbers: Vec<i32> = vec![1, 2, 3];

    // Slice (borrowed)
    let slice = &numbers[..];  // Borrow whole vector as slice
    println!("first slice: {:?}", slice);   // Prints: [1, 2, 3]

    // Mutable vector
    let mut numbers = vec![1, 2, 3];

    // It's ok to have immutable borrow while no mutable exist yet.
    let another_slice = &numbers[..];
    println!("another_slice: {:?}", another_slice);

    // Mutable slice
    let slice = &mut numbers[..];
    slice[0] = 10;  // OK: modifying through mutable slice
    println!("slice: {:?}",slice);
    println!("numbers: {:?}", numbers); // immutable borrow here

    // This would fail:
    // let another_slice = &numbers[..];  //Can't have immutable borrow
                                         //while mutable borrow exists


}
