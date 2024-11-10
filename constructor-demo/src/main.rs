#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

// the constructor function name doesn't have to be new in Rust,
// but using new is a strong convention in the Rust community.

// While you can name your constructor function anything you want,
// using new is considered idiomatic Rust because:
// It's widely recognized by Rust developers
// It follows the standard library's pattern
// It makes code more readable and predictable

impl User {
    fn new(username:String, email:String, uri:String) -> Self {
        Self{
            username,
            email,
            uri,
            active: true    //default value
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    // Alternative constructor names are possible
    fn create(username:String, email:String, uri:String) -> Self {
        Self{
            username,
            email,
            uri,
            active: true    //default value
        }
    }
}


fn main() {
    let mut user1 = User::new(
        String::from("jason"),
        String::from("jason@example.com"),
        String::from("www.example.com"),
    );

    user1.deactivate();

    if user1.active{
        println!("User1: {:?}", user1);
    }
    else {
        println!("{:?} is not activated.", user1);
    }

}
