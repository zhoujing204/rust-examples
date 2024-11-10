use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }

    // Get the file path from arguments
    let file_path = &args[1];

    // Try to read the file and handle potential errors
    match read_file(file_path) {
        Ok(()) => println!("File read successfully!"),
        Err(error) => println!("Error reading file: {}", error),
    }
}

fn read_file(file_path: &str) -> io::Result<()> {
    // Check if file exists
    if !Path::new(file_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist"
        ));
    }

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read and print each line
    println!("File contents of {}:", file_path);
    println!("-------------------");

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => println!("{}: {}", index + 1, content),
            Err(error) => println!("Error reading line {}: {}", index + 1, error),
        }
    }

    println!("-------------------");
    Ok(())
}