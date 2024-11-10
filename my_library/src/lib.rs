pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn new_function() -> String {
    "Hello".to_string()
}

pub struct User {
    pub name: String,
    age: i32  // Private field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
