fn main() {

    let number = 3;
    let return_value = own_integer(number);
    println!("{}", return_value);

    let my_string = String::from("Hello World.");
    // my_string ownership is moved
    own_string(my_string);
    //println!("{}", my_string);

    let my_string2 = String::from("Second String.");
    //reference a String, ownership is not moved.
    ref_string(&my_string2);

}

// reference a String
fn ref_string(s: &String) {
    println!("{}", s);
}

// integer is copied, not moved
fn own_integer(x: i32) -> i32 {
    println!("{}", x);
    return x+1;
}

fn own_string(s: String) {
    println!("{}", s);
}
