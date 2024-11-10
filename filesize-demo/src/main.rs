enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64)
}

impl FileSize {
    // Static functions (called with ::)
    fn from_bytes(bytes: u64) -> FileSize {
        if bytes >= 1024 * 1024 * 1024 {
            FileSize::Gigabytes(bytes / (1024 * 1024 * 1024))
        } else if bytes >= 1024 * 1024 {
            FileSize::Megabytes(bytes / (1024 * 1024))
        } else if bytes >= 1024 {
            FileSize::Kilobytes(bytes / 1024)
        } else {
            FileSize::Bytes(bytes)
        }
    }

    // Another static function example
    fn zero() -> FileSize {
        FileSize::Bytes(0)
    }

    // Your existing methods (with &self)
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(b) => format!("{} B", b),
            FileSize::Kilobytes(kb) => format!("{} KB", kb),
            FileSize::Megabytes(mb) => format!("{} MB", mb),
            FileSize::Gigabytes(gb) => format!("{} GB", gb)
        }
    }

    fn to_bytes(&self) -> u64 {
        match self {
            FileSize::Bytes(b) => *b,
            FileSize::Kilobytes(kb) => kb * 1024,
            FileSize::Megabytes(mb) => mb * 1024 * 1024,
            FileSize::Gigabytes(gb) => gb * 1024 * 1024 * 1024
        }
    }
}

fn main() {
    // Using static functions (with ::)
    let size1 = FileSize::from_bytes(1_500_000);
    let size2 = FileSize::zero();

    // Using methods (with .)
    println!("Size 1: {}", size1.format_size()); // prints "1 MB"
    println!("Size 2: {}", size2.format_size()); // prints "0 B"

    // Create different file sizes
    let small_file = FileSize::Bytes(800);
    let medium_file = FileSize::Kilobytes(123);
    let large_file = FileSize::Megabytes(45);
    let huge_file = FileSize::Gigabytes(2);

    // Use format_size() method
    println!("Small file size: {}", small_file.format_size());   // 800 B
    println!("Medium file size: {}", medium_file.format_size()); // 123 KB
    println!("Large file size: {}", large_file.format_size());   // 45 MB
    println!("Huge file size: {}", huge_file.format_size());     // 2 GB

    // Demonstrate to_bytes() conversion
    println!("Large file in bytes: {}", large_file.to_bytes()); // 47185920
}