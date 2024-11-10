/// Adds two unsigned 64-bit integers
///
/// # Arguments
/// * `left` - First number to add
/// * `right` - Second number to add
///
/// # Returns
/// Sum of the two numbers
///
/// # Examples
/// ```
/// use my_library::add;  // replace my_library with your crate name
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Returns a String containing "Hello"
///
/// # Returns
/// A new String with "Hello"
///
/// # Examples
/// ```
/// use my_library::new_function;
/// let greeting = new_function();
/// assert_eq!(greeting, "Hello");
/// ```
pub fn new_function() -> String {
    "Hello".to_string()
}

/// Represents a user with name and age
///
/// # Fields
/// * `name` - User's name (public)
/// * `age` - User's age (private)
///
/// # Examples
/// ```
/// use my_library::User;
/// let user = User {
///     name: String::from("John"),
///     age: 30,
/// };
/// assert_eq!(user.name, "John");
/// ```
pub struct User {
    pub name: String,
    pub age: i32
}


pub mod colors;