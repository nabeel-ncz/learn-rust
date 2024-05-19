enum Direction {
    North,
    South,
    East,
    West
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

fn calculate_area(shape: Shape) -> f64 {
    let ans: f64 = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(s) => s * s,
        Shape::Rectangle(w, h) => w * h,
    }
    return ans;
}

fn main() {
    println!("Hello, world!");

    // Enums are similar to enums in typescript
    // it allows to define a type of different variants
    // for example

    let my_dir = Direction::North;


    //we can also define enums with value
    // that means whenever it's calling we should pass some value
    // it expect some value from us for example the Shape enum
    
    let circle = Shape::Circle(10.00);
    let square = Shape::Square(1000.982);
    let rectangle = Shape::Rectangle(8.00, 4.0);

    // here the new topic comes in to picture
    // =======pattern matching==============

    // pattern match across various variants of an enum and run some logic
    // eg -> calculate_area

    println!("{}", calculate_area(circle));

}
