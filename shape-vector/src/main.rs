// First, define an enum for shapes
enum Shape {
    Circle(f64),      // radius
    Square(f64),      // side length
    Rectangle(f64, f64) // width, height
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width * height
        }
    }
}

fn main() {
    // Create a vector of shapes
    let shapes: Vec<Shape> = vec![
        Shape::Circle(5.0),
        Shape::Square(4.0),
        Shape::Rectangle(3.0, 6.0),
        Shape::Circle(2.5)
    ];

    // Different ways to work with the vector:

    // 1. Iterate over shapes
    for shape in &shapes {
        println!("Area: {:.2}", shape.area());
    }

    // 2. Access by index
    let first_shape = &shapes[0];

    // 3. Vector methods
    println!("Number of shapes: {}", shapes.len());

    // 4. Filter specific shapes
    let circles: Vec<&Shape> = shapes.iter()
        .filter(|shape| matches!(shape, Shape::Circle(_)))
        .collect();

    // 5. Find largest area
    let largest_area = shapes.iter()
        .map(|shape| shape.area())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
}