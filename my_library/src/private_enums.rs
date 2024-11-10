// All variants inherit enum's visibility
pub enum Example {
    Public(String),           // No pub needed on fields
    AnotherPublic(i32, bool),
}

// Private enum - only accessible within module
enum PrivateExample {
    PrivateVariant,
    AnotherPrivate,
}

// Struct-like enum variants
pub enum ComplexExample {
    StructLike {              // No pub needed on fields
        public_field: String,
        private_field: i32,
    },
    TupleLike(String, i32)    // Tuple variant
}