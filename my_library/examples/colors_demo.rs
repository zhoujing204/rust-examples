use my_library::colors::{red, green, bold};

fn main() {
    println!("{}", red("Colors_demo: This is an error message"));
    println!("{}", green("Colors_demo: This is a success message"));
    println!("{}", bold("Colors_demo: This is important text"));
}