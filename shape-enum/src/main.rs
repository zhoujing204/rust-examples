enum Shape {
    Circle(f64),              // Tuple-style
    Square,                   // Unit-style
    Rectangle {              // Struct-style
        width: f64,
        height: f64
    }
}

fn main() {
    // Creating instances
    let circle = Shape::Circle(5.0);
    let square = Shape::Square;
    let rectangle = Shape::Rectangle {
        width: 10.0,
        height: 20.0
    };

    // Pattern matching
    match rectangle {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Square => println!("A square"),
        Shape::Rectangle { width, height } => println!("Rectangle {}×{}", width, height)
    }
}